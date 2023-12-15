use std::io::{Read, Write};
use std::net::TcpStream;

pub fn run_client(port: &str) {
    let server_address = format!("127.0.0.1:{}", port);
    match TcpStream::connect(&server_address) {
        Ok(mut stream) => {
            let handshake_request = format!(
                "GET / HTTP/1.1\r\n\
                 Host: {}\r\n\
                 Upgrade: websocket\r\n\
                 Connection: Upgrade\r\n\
                 Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\r\n\
                 Sec-WebSocket-Version: 13\r\n\r\n",
                server_address
            );
            stream.write_all(handshake_request.as_bytes()).unwrap();
            stream.flush().unwrap();

            let mut response = [0; 1024];
            stream.read(&mut response).unwrap();
        }
        Err(e) => eprintln!("Failed to connect: {}", e),
    }
}
