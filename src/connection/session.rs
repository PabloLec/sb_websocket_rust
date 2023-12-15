use std::io;
use std::io::Write;
use std::net::{SocketAddr, TcpStream};

use crate::connection::stream::read_stream;
use crate::websocket::{decode_frame, encode_frame};

pub struct Session {
    pub stream: TcpStream,
    pub peer_addr: SocketAddr,
    pub received_messages: Vec<String>,
    pub sent_messages: Vec<String>,
    pub is_alive: bool,
}

impl Session {
    pub fn new(stream: TcpStream) -> Self {
        let peer_addr = stream.peer_addr().unwrap();
        Self {
            stream,
            peer_addr,
            received_messages: Vec::new(),
            sent_messages: Vec::new(),
            is_alive: false,
        }
    }

    pub fn start(&mut self) {
        println!("New Session: {}", self.peer_addr);
        self.is_alive = true;
        let mut sent_messages_count = self.sent_messages.len();

        while self.is_alive {
            let mut input = String::new();
            let n = io::stdin().read_line(&mut input).unwrap();

            if n > 0 {
                self.stream.write_all(&encode_frame(&input)).unwrap();
            }

            match read_stream(&mut self.stream) {
                Ok(message_buffer) => {
                    println!(
                        "Received data: {}",
                        decode_frame(&message_buffer)
                            .unwrap_or_else(|_| "Invalid UTF-8".to_string())
                    );
                    self.received_messages.push(
                        decode_frame(&message_buffer)
                            .unwrap_or_else(|_| "Invalid UTF-8".to_string()),
                    );
                }
                Err(e) => eprintln!("Error reading message: {}", e),
            }
        }
    }

    pub fn send_message(&mut self, message: String) {
        self.sent_messages.push(message);
    }
}
