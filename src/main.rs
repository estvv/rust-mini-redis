mod dispatcher;
mod request;
mod returns;
mod stock;

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use crate::dispatcher::Dispatcher;
use crate::request::Request;
use crate::returns::Return;

fn read_success(
    dispatcher: Arc<Mutex<Dispatcher>>,
    taille: usize,
    buffer: &[u8],
    stream: &mut TcpStream,
) -> bool {
    if taille == 0 {
        println!("Client disconnected.");
        return false;
    }

    let requete = String::from_utf8_lossy(&buffer[..taille]);

    print!("Received: {}", requete);

    let response = match Request::parse(&requete) {
        Ok(request) => match dispatcher.lock().unwrap().dispatch(request) {
            Return::Ok(value) => format!("{}\r\n", value),
            Return::Err(err) => format!("Error: {}\r\n", err),
            Return::NotFound(key) => format!("Key '{}' not found\r\n", key),
        },
        Err(err) => format!("Error: {}\r\n", err),
    };

    stream.write_all(response.as_bytes()).unwrap();
    true
}

fn read_stream(mut stream: TcpStream, dispatcher: Arc<Mutex<Dispatcher>>) {
    let mut buffer = [0; 512];

    loop {
        match stream.read(&mut buffer) {
            Ok(taille) => {
                if !read_success(dispatcher.clone(), taille, &buffer, &mut stream) {
                    break;
                }
            }
            Err(e) => {
                println!("Read error: {}", e);
                break;
            }
        }
    }
}

fn stream_success(stream: TcpStream, dispatcher: Arc<Mutex<Dispatcher>>) {
    println!("New connection established!");

    thread::spawn(move || {
        read_stream(stream, dispatcher);
    });
}

fn stream_error(e: std::io::Error) {
    println!("Connection error: {}", e);
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").expect("Unable to open port");
    println!("Mini-Redis started on port 6379.");
    println!("Waiting for connections...");

    let dispatcher = Arc::new(Mutex::new(Dispatcher::new()));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => stream_success(stream, Arc::clone(&dispatcher)),
            Err(e) => stream_error(e),
        }
    }
}
