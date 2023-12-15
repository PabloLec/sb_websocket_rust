use std::net::TcpStream;

use crate::client::handshake::handle_handshake;
use crate::connection::session::Session;

pub fn run_client(port: &str) {
    let server_address = format!("127.0.0.1:{}", port);
    match TcpStream::connect(&server_address) {
        Ok(mut stream) => match handle_handshake(&mut stream, server_address) {
            Ok(_) => {
                Session::new(stream).start();
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        },
        Err(e) => eprintln!("Failed to connect: {}", e),
    }
}
