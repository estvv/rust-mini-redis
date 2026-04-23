mod dispatcher;
mod request;
mod returns;
mod stock;

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use crate::dispatcher::Dispatcher;
use crate::request::Request;
use crate::returns::Return;

fn read_success(dispatcher: Arc<Mutex<Dispatcher>>, line: String, stream: &mut TcpStream) -> bool {
    if line.is_empty() {
        return true;
    }

    print!("Received: {}", line);

    let response = match Request::parse(&line) {
        Ok(request) => match dispatcher.lock().unwrap().dispatch(request) {
            Return::Ok(value) => format!("{}\r\n", value),
            Return::Err(err) => format!("Error: {}\r\n", err),
            Return::NotFound(key) => format!("Key '{}' not found\r\n", key),
        },
        Err(err) => format!("Error: {}\r\n", err),
    };

    if stream.write_all(response.as_bytes()).is_err() {
        println!("Write error.");
        return false;
    }
    true
}

fn read_stream(
    mut stream: TcpStream,
    dispatcher: Arc<Mutex<Dispatcher>>,
    running: Arc<AtomicBool>,
) {
    stream
        .set_read_timeout(Some(std::time::Duration::from_millis(100)))
        .ok();
    let mut buffer = Vec::new();
    let mut byte = [0u8; 1];

    loop {
        if !running.load(Ordering::SeqCst) {
            println!("Client thread shutting down...");
            return;
        }

        match stream.read(&mut byte) {
            Ok(0) => {
                println!("Client disconnected.");
                return;
            }
            Ok(_) => {
                buffer.push(byte[0]);
                if byte[0] == b'\n' {
                    let line = String::from_utf8_lossy(&buffer).into_owned();
                    buffer.clear();
                    if !read_success(dispatcher.clone(), line, &mut stream) {
                        return;
                    }
                }
            }
            Err(ref e)
                if e.kind() == std::io::ErrorKind::WouldBlock
                    || e.kind() == std::io::ErrorKind::TimedOut =>
            {
                continue;
            }
            Err(e) => {
                println!("Read error: {}", e);
                return;
            }
        }
    }
}

fn stream_success(stream: TcpStream, dispatcher: Arc<Mutex<Dispatcher>>, running: Arc<AtomicBool>) {
    println!("New connection established!");

    thread::spawn(move || {
        read_stream(stream, dispatcher, running);
    });
}

fn stream_error(e: std::io::Error) {
    println!("Connection error: {}", e);
}

fn main() {
    let running = Arc::new(AtomicBool::new(true));

    ctrlc::set_handler({
        let running = running.clone();
        move || {
            println!("\nShutdown signal received...");
            running.store(false, Ordering::SeqCst);
        }
    })
    .expect("Error setting Ctrl+C handler");

    let listener = TcpListener::bind("127.0.0.1:6379").expect("Unable to open port");
    listener
        .set_nonblocking(true)
        .expect("Failed to set non-blocking");

    println!("Mini-Redis started on port 6379.");
    println!("Press Ctrl+C to shutdown.");
    println!("Waiting for connections...");

    let dispatcher = Arc::new(Mutex::new(Dispatcher::new()));

    while running.load(Ordering::SeqCst) {
        match listener.accept() {
            Ok((stream, _)) => stream_success(stream, Arc::clone(&dispatcher), running.clone()),
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                thread::sleep(std::time::Duration::from_millis(100));
                continue;
            }
            Err(e) => stream_error(e),
        }
    }

    println!("Shutting down gracefully...");
    println!("Server stopped.");
}
