use std::io::Write;

use crate::connection::stream::read_stream;
use crate::websocket::{compute_key_hash, generate_key};

pub fn handle_handshake(
    stream: &mut std::net::TcpStream,
    server_address: String,
) -> Result<(), &'static str> {
    let key = generate_key();

    send_request(stream, &key, &server_address)?;

    match read_stream(stream) {
        Ok(response_buffer) => {
            let response = String::from_utf8_lossy(&response_buffer);
            handle_response(&response, &key)
        }
        Err(_) => {
            Err("Error reading response")
        }
    }
}

fn send_request(
    mut stream: &std::net::TcpStream,
    key: &str,
    server_address: &str,
) -> Result<(), &'static str> {
    let handshake_request = format!(
        "GET / HTTP/1.1\r\n\
                 Host: {}\r\n\
                 Upgrade: websocket\r\n\
                 Connection: Upgrade\r\n\
                 Sec-WebSocket-Key: {}\r\n\
                 Sec-WebSocket-Version: 13\r\n\r\n",
        server_address, key
    );
    stream.write_all(handshake_request.as_bytes()).unwrap();
    stream.flush().unwrap();

    Ok(())
}

fn handle_response(response: &str, key: &str) -> Result<(), &'static str> {
    if response.contains("HTTP/1.1 101") && response.contains(&compute_key_hash(key)) {
        Ok(())
    } else {
        eprintln!("Handshake failed");
        Err("Handshake failed")
    }
}
