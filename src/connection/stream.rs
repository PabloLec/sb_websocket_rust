use std::io::Read;
use std::net::TcpStream;

pub fn read_stream(stream: &mut TcpStream) -> Result<Vec<u8>, std::io::Error> {
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
