use base64::{engine::general_purpose, Engine as _};
use sha1::{Digest, Sha1};
use std::io::Write;

pub fn handle_handshake(
    mut stream: &std::net::TcpStream,
    request: &str,
) -> Result<(), &'static str> {
    if request.starts_with("GET") && request.contains("Upgrade: websocket") {
        let key = extract_key(request);
        if key.is_empty() {
            return Err("Invalid WebSocket Key");
        }

        let accept_val = compute_websocket_accept(&key);

        let response = format!(
            "HTTP/1.1 101 Switching Protocols\r\nConnection: Upgrade\r\nUpgrade: websocket\r\nSec-WebSocket-Accept: {}\r\n\r\n",
            accept_val
        );
        println!("Response: {}", response);
        stream
            .write_all(response.as_bytes())
            .map_err(|_| "Failed to send handshake response")?;
        stream.flush().map_err(|_| "Failed to flush stream")?;

        Ok(())
    } else {
        Err("Invalid WebSocket Request")
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

fn compute_websocket_accept(key: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.update(key.as_bytes());
    hasher.update(b"258EAFA5-E914-47DA-95CA-C5AB0DC85B11");
    general_purpose::STANDARD.encode(hasher.finalize())
}
