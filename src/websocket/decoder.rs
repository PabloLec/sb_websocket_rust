pub fn decode_frame(frame: &[u8]) -> Result<String, &'static str> {
    if frame.len() < 2 {
        return Err("Frame too short");
    }

    let is_masked = frame[1] & 0b10000000 != 0;
    if !is_masked {
        return Err("Expected masked frame");
    }

    let mut data_length = (frame[1] & 0b01111111) as usize;
    let mut offset = 2;

    if data_length == 126 {
        if frame.len() < 4 {
            return Err("Frame too short for 126 length");
        }
        data_length = u16::from_be_bytes([frame[2], frame[3]]) as usize;
        offset += 2;
    } else if data_length == 127 {
        return Err("128+ length frames not supported");
    }

    if frame.len() < offset + 4 + data_length {
        return Err("Frame too short for data");
    }

    let masking_key = &frame[offset..offset + 4];
    offset += 4;

    let mut decoded = Vec::with_capacity(data_length);
    for i in 0..data_length {
        decoded.push(frame[offset + i] ^ masking_key[i % 4]);
    }

    String::from_utf8(decoded).map_err(|_| "Invalid UTF-8")
}
