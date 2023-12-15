use rand::{distributions::Alphanumeric, Rng};

pub fn encode_frame(message: &str) -> Vec<u8> {
    let mut frame = Vec::new();

    frame.push(0x81);

    let payload = message.as_bytes();
    let payload_len = payload.len();
    let mask: Vec<u8> = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(4)
        .collect();

    if payload_len <= 125 {
        frame.push(0x80 | payload_len as u8);
    } else if payload_len <= 65535 {
        frame.push(0x80 | 126);
        frame.extend_from_slice(&(payload_len as u16).to_be_bytes());
    } else {
        frame.push(0x80 | 127);
        frame.extend_from_slice(&(payload_len as u64).to_be_bytes());
    }

    frame.extend_from_slice(&mask);

    frame.extend(
        payload
            .iter()
            .enumerate()
            .map(|(i, &byte)| byte ^ mask[i % 4]),
    );

    frame
}
