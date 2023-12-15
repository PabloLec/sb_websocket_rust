use crate::connection::session::Session;
use crate::connection::stream::read_stream;
use crate::server::handshake::handle_handshake;
use std::net::{TcpListener, TcpStream};
use std::thread;

pub fn run_server() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Cannot bind to port");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    println!("New connection: {}", stream.peer_addr().unwrap());
                    handle_connection(stream);
                });
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    match read_stream(&mut stream) {
        Ok(request_buffer) => {
            let request = String::from_utf8_lossy(&request_buffer);
            println!("Received request: {}", request);

            match handle_handshake(&stream, &request) {
                Ok(_) => {
                    println!("WebSocket Handshake Successful");
                    Session::new(stream).start();
                }
                Err(e) => eprintln!("WebSocket Handshake Failed: {}", e),
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
