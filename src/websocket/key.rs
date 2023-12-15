use base64::engine::general_purpose;
use base64::Engine;
use rand::distributions::Alphanumeric;
use rand::Rng;
use sha1::{Digest, Sha1};

pub fn compute_key_hash(key: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.update(key.as_bytes());
    hasher.update(b"258EAFA5-E914-47DA-95CA-C5AB0DC85B11");
    general_purpose::STANDARD.encode(hasher.finalize())
}

pub fn generate_key() -> String {
    let random_bytes: Vec<u8> = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .collect();
    general_purpose::STANDARD.encode(random_bytes)
}
