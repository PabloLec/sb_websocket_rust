use std::net::TcpStream;
use std::io::Read;

use crate::server::handshake::handle_handshake;
use crate::messaging::decode_frame;

pub fn handle_connection(mut stream: TcpStream) {
    match read_stream(&mut stream) {
        Ok(request_buffer) => {
            let request = String::from_utf8_lossy(&request_buffer);
            println!("Received request: {}", request);

            match handle_handshake(&stream, &request) {
                Ok(_) => {
                    println!("WebSocket Handshake Successful");
                    match read_stream(&mut stream) {
                        Ok(message_buffer) => {
                            println!("Received data: {}", decode_frame(&message_buffer).unwrap_or_else(|_| "Invalid UTF-8".to_string()));
                        },
                        Err(e) => eprintln!("Error reading message: {}", e),
                    }
                },
                Err(e) => eprintln!("WebSocket Handshake Failed: {}", e),
            }
        },
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn read_stream(stream: &mut TcpStream) -> Result<Vec<u8>, std::io::Error> {
    let mut buffer = Vec::new();
    let mut temp_buffer = [0; 1024];
    loop {
        let n = stream.read(&mut temp_buffer)?;
        if n == 0 {
            break;
        }
        buffer.extend_from_slice(&temp_buffer[..n]);
        if n < temp_buffer.len() {
            break;
        }
    }
    Ok(buffer)
}
