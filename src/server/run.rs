use std::io;
use std::net::{TcpListener, TcpStream};

use crate::connection::session::Session;
use crate::server::handshake::handle_handshake;

pub fn run_server(port: &str) -> Result<Session, io::Error> {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))?;
    println!("Waiting for connection with peer");
    match listener.accept() {
        Ok((stream, _addr)) => handle_connection(stream),
        Err(e) => {
            eprintln!("Error: {}", e);
            Err(e)
        }
    }
}

fn handle_connection(mut stream: TcpStream) -> Result<Session, io::Error> {
    match handle_handshake(&mut stream) {
        Ok(_) => {
            let mut session = Session::new(stream);
            session.start();
            Ok(session)
        }
        Err(e) => Err(io::Error::new(io::ErrorKind::Other, e)),
    }
}
