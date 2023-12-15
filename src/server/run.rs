use std::net::TcpListener;
use std::thread;
use crate::server::connection::handle_connection;

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