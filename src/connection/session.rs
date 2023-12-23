use crate::connection::Message::Message;
use std::io::Write;
use std::net::{SocketAddr, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

use crate::connection::stream::{read_stream, read_stream_non_blocking};
use crate::websocket::{decode_frame, encode_frame};

pub struct Session {
    pub stream: Arc<Mutex<TcpStream>>,
    pub peer_addr: SocketAddr,
    pub messages: Arc<Mutex<Vec<Message>>>,
    pub is_alive: Arc<Mutex<bool>>,
}

impl Session {
    pub fn new(stream: TcpStream) -> Self {
        let peer_addr = stream.peer_addr().unwrap();
        Self {
            stream: Arc::new(Mutex::new(stream)),
            peer_addr,
            messages: Arc::new(Mutex::new(vec![])),
            is_alive: Arc::new(Mutex::new(false)),
        }
    }

    pub fn start(&mut self) {
        self.is_alive = Arc::new(Mutex::new(true));
        let messages = self.messages.clone();
        let is_alive = self.is_alive.clone();
        let stream = self.stream.clone();

        thread::spawn(move || loop {
            {
                let is_alive_status = *is_alive.lock().unwrap();
                if !is_alive_status {
                    break;
                }
            }

            let mut stream_lock = stream.lock().unwrap();
            match read_stream_non_blocking(&mut stream_lock) {
                Ok(message_buffer) => {
                    if message_buffer.is_empty() {
                        continue;
                    }
                    messages.lock().unwrap().push(Message::receive_new(
                        decode_frame(&message_buffer)
                            .unwrap_or_else(|_| "Invalid UTF-8".to_string()),
                    ));
                }
                Err(e) => eprintln!("Error reading message: {}", e),
            }
        });
    }

    pub fn send_message(&self, message: String) {
        let mut stream_lock = self.stream.lock().unwrap();
        stream_lock.write_all(&encode_frame(&message)).unwrap();
        self.messages
            .lock()
            .unwrap()
            .push(Message::send_new(message));
    }

    pub fn get_messages(&self) -> Vec<Message> {
        let messages = self.messages.lock().unwrap();
        messages.clone()
    }
}
