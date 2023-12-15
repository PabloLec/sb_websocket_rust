use std::io::Write;

use crate::connection::stream::read_stream;
use crate::websocket::compute_key_hash;

pub fn handle_handshake(mut stream: &mut std::net::TcpStream) -> Result<(), &'static str> {
    match read_stream(&mut stream) {
        Ok(request_buffer) => {
            let request = String::from_utf8_lossy(&request_buffer);

            if request.starts_with("GET") && request.contains("Upgrade: websocket") {
                let key = extract_key(&request);
                if key.is_empty() {
                    return Err("Invalid WebSocket Key");
                }
                let accept_val = compute_key_hash(&key);

                send_response(&stream, &accept_val)
            } else {
                Err("Invalid WebSocket Request")
            }
        }
        Err(_) => Err("Error reading stream"),
    }
}

fn extract_key(request: &str) -> String {
    request
        .lines()
        .find(|line| line.starts_with("Sec-WebSocket-Key:"))
        .and_then(|line| line.split(": ").nth(1))
        .map(|value| value.trim().to_string())
        .unwrap_or_else(|| "".to_string())
}

fn send_response(mut stream: &std::net::TcpStream, accept_val: &str) -> Result<(), &'static str> {
    let response = format!(
        "HTTP/1.1 101 Switching Protocols\r\nConnection: Upgrade\r\nUpgrade: websocket\r\nSec-WebSocket-Accept: {}\r\n\r\n",
        accept_val
    );

    stream
        .write_all(response.as_bytes())
        .map_err(|_| "Failed to send handshake response")?;
    stream.flush().map_err(|_| "Failed to flush stream")?;
    Ok(())
}
