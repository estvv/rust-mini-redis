// src/main.rs

mod dispatcher;
mod channel_manager;
mod request;
mod returns;
mod stock;

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{broadcast, Mutex};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use crate::dispatcher::Dispatcher;
use crate::request::Request;
use crate::returns::Return;

static CLIENT_ID_COUNTER: AtomicU64 = AtomicU64::new(1);

async fn read_success<W: AsyncWriteExt + Unpin>(dispatcher: Arc<Mutex<Dispatcher>>, line: String, writer: &mut W, client_id: u64) -> Option<broadcast::Receiver<String>> {
    if line.is_empty() {
        return None;
    }

    print!("Received: {}", line);

    let result = match Request::parse(&line) {
        Ok(request) => dispatcher.lock().await.dispatch(request, client_id),
        Err(err) => Return::Err(err),
    };

    let response = match result {
        Return::Ok(value) => format!("{}\r\n", value),
        Return::Err(err) => format!("Error: {}\r\n", err),
        Return::NotFound(key) => format!("Key '{}' not found\r\n", key),
        Return::Subscribe(receiver) => {
            let _ = writer.write_all(b"Subscribed\r\n").await;
            return Some(receiver);
        }
        Return::Unsubscribe => {
            let _ = writer.write_all(b"Unsubscribed\r\n").await;
            return None;
        }
    };

    if writer.write_all(response.as_bytes()).await.is_err() {
        println!("Write error.");
        return None;
    }
    None
}

async fn handle_client(stream: TcpStream, dispatcher: Arc<Mutex<Dispatcher>>) {
    let client_id = CLIENT_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);
    let mut buffer = String::new();
    let mut subscription: Option<broadcast::Receiver<String>> = None;

    loop {
        buffer.clear();

        if let Some(ref mut rx) = subscription {
            tokio::select! {
                result = reader.read_line(&mut buffer) => {
                    match result {
                        Ok(0) => {
                            println!("Client {} disconnected.", client_id);
                            dispatcher.lock().await.cleanup_client(client_id);
                            return;
                        }
                        Ok(_) => {
                            let result = read_success(dispatcher.clone(), buffer.clone(), &mut writer, client_id).await;
                            match result {
                                Some(rx) => subscription = Some(rx),
                                None => subscription = None,
                            }
                        }
                        Err(e) => {
                            println!("Read error: {}", e);
                            dispatcher.lock().await.cleanup_client(client_id);
                            return;
                        }
                    }
                }
                result = rx.recv() => {
                    match result {
                        Ok(msg) => {
                            if writer.write_all(format!("{}\r\n", msg).as_bytes()).await.is_err() {
                                println!("Write error.");
                                dispatcher.lock().await.cleanup_client(client_id);
                                return;
                            }
                        }
                        Err(broadcast::error::RecvError::Closed) => {
                            println!("Client {}: Channel closed.", client_id);
                            subscription = None;
                        }
                        Err(broadcast::error::RecvError::Lagged(_)) => {
                            continue;
                        }
                    }
                }
            }
        } else {
            match reader.read_line(&mut buffer).await {
                Ok(0) => {
                    println!("Client {} disconnected.", client_id);
                    dispatcher.lock().await.cleanup_client(client_id);
                    return;
                }
                Ok(_) => {
                    if let Some(rx) = read_success(dispatcher.clone(), buffer.clone(), &mut writer, client_id).await {
                        subscription = Some(rx);
                    }
                }
                Err(e) => {
                    println!("Read error: {}", e);
                    dispatcher.lock().await.cleanup_client(client_id);
                    return;
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379")
        .await
        .expect("Unable to open port");

    println!("Started on port 6379.");
    println!("Press Ctrl+C to shutdown.");
    println!("Waiting for connections...");

    let dispatcher = Arc::new(Mutex::new(Dispatcher::new()));

    loop {
        tokio::select! {
            result = listener.accept() => {
                match result {
                    Ok((stream, _)) => {
                        println!("New connection established!");
                        let dispatcher = Arc::clone(&dispatcher);
                        tokio::spawn(async move {
                            handle_client(stream, dispatcher).await;
                        });
                    }
                    Err(e) => {
                        println!("Connection error: {}", e);
                    }
                }
            }
            _ = tokio::signal::ctrl_c() => {
                println!("\nShutdown signal received...");
                println!("Shutting down gracefully...");
                println!("Server stopped.");
                return;
            }
        }
    }
}
