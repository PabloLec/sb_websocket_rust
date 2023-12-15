use std::net::{TcpListener, TcpStream};
use std::thread;

use crate::connection::session::Session;
use crate::server::handshake::handle_handshake;

pub fn run_server(port: &str) {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).expect("Cannot bind to port");

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
    match handle_handshake(&mut stream) {
        Ok(_) => {
            Session::new(stream).start();
        }
        Err(e) => eprintln!("Connection failed: {}", e),
    }
}
