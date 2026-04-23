mod dispatcher;
mod request;
mod returns;
mod stock;

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use crate::dispatcher::Dispatcher;
use crate::request::Request;
use crate::returns::Return;

fn handle_clients(mut stream: TcpStream, dispatcher: &mut Dispatcher) {
    let mut buffer = [0; 512];

    loop {
        match stream.read(&mut buffer) {
            Ok(taille) => {
                if taille == 0 {
                    println!("Client disconnected.");
                    break;
                }

                let requete = String::from_utf8_lossy(&buffer[..taille]);

                print!("Received: {}", requete);

                let response = match Request::parse(&requete) {
                    Ok(request) => match dispatcher.dispatch(request) {
                        Return::Ok(value) => format!("{}\r\n", value),
                        Return::Err(err) => format!("Error: {}\r\n", err),
                        Return::NotFound(key) => format!("Key '{}' not found\r\n", key),
                    },
                    Err(err) => format!("Error: {}\r\n", err),
                };

                stream.write_all(response.as_bytes()).unwrap();
            }
            Err(e) => {
                println!("Read error: {}", e);
                break;
            }
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").expect("Unable to open port");
    println!("Mini-Redis started on port 6379.");
    println!("Waiting for connections...");

    let mut dispatcher = Dispatcher::new();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection established!");
                handle_clients(stream, &mut dispatcher);
            }
            Err(e) => {
                println!("Connection error: {}", e);
            }
        }
    }
}
