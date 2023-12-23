use std::io;
use std::net::TcpStream;

use crate::client::handshake::handle_handshake;
use crate::connection::session::Session;

pub fn run_client(port: &str) -> Result<Session, io::Error> {
    let server_address = format!("127.0.0.1:{}", port);
    let mut stream = TcpStream::connect(&server_address)?;

    match handle_handshake(&mut stream, server_address) {
        Ok(_) => {
            let mut session = Session::new(stream);
            session.start();
            Ok(session)
        }
        Err(e) => Err(io::Error::new(io::ErrorKind::Other, e)),
    }
}
