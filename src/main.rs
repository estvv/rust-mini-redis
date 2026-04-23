mod dispatcher;
mod request;
mod returns;
mod stock;

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use std::sync::Arc;

use crate::dispatcher::Dispatcher;
use crate::request::Request;
use crate::returns::Return;

async fn read_success<W: AsyncWriteExt + Unpin>(
    dispatcher: Arc<Mutex<Dispatcher>>,
    line: String,
    writer: &mut W,
) -> bool {
    if line.is_empty() {
        return true;
    }

    print!("Received: {}", line);

    let response = match Request::parse(&line) {
        Ok(request) => match dispatcher.lock().await.dispatch(request) {
            Return::Ok(value) => format!("{}\r\n", value),
            Return::Err(err) => format!("Error: {}\r\n", err),
            Return::NotFound(key) => format!("Key '{}' not found\r\n", key),
        },
        Err(err) => format!("Error: {}\r\n", err),
    };

    if writer.write_all(response.as_bytes()).await.is_err() {
        println!("Write error.");
        return false;
    }
    true
}

async fn handle_client(
    stream: TcpStream,
    dispatcher: Arc<Mutex<Dispatcher>>,
) {
    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);
    let mut buffer = String::new();

    loop {
        buffer.clear();
        match reader.read_line(&mut buffer).await {
            Ok(0) => {
                println!("Client disconnected.");
                return;
            }
            Ok(_) => {
                if !read_success(dispatcher.clone(), buffer.clone(), &mut writer).await {
                    return;
                }
            }
            Err(e) => {
                println!("Read error: {}", e);
                return;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379")
        .await
        .expect("Unable to open port");

    println!("Mini-Redis started on port 6379.");
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