

pub fn pad_bytes(unpadded: &[u8], block_length: usize, padding: u8) -> Vec<u8> {
    let mut padded_vec = unpadded.to_vec();
    let length = padded_vec.len();
    let bytes_needed = block_length - length;
    for _ in 0..bytes_needed {
        padded_vec.push(padding);
    }
    padded_vec
}
