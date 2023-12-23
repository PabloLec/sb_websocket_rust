#[derive(Clone)]
pub struct Message {
    pub body: String,
    pub is_sent: bool,
    pub timestamp: std::time::Instant,
}

impl Message {
    pub fn send_new(body: String) -> Message {
        Message {
            body,
            is_sent: true,
            timestamp: std::time::Instant::now(),
        }
    }

    pub fn receive_new(body: String) -> Message {
        Message {
            body,
            is_sent: false,
            timestamp: std::time::Instant::now(),
        }
    }
}
