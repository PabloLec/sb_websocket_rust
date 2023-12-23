use std::io;
use std::io::Read;
use std::net::TcpStream;

pub fn read_stream(stream: &mut TcpStream) -> Result<Vec<u8>, io::Error> {
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

pub fn read_stream_non_blocking(stream: &mut TcpStream) -> io::Result<Vec<u8>> {
    stream.set_nonblocking(true)?;

    let mut buffer = Vec::new();
    let mut temp_buffer = [0; 1024];
    match stream.read(&mut temp_buffer) {
        Ok(0) => Ok(buffer),
        Ok(n) => {
            buffer.extend_from_slice(&temp_buffer[..n]);
            Ok(buffer)
        }
        Err(ref e)
            if e.kind() == io::ErrorKind::WouldBlock || e.kind() == io::ErrorKind::Interrupted =>
        {
            Ok(buffer)
        }
        Err(e) => Err(e),
    }
}
