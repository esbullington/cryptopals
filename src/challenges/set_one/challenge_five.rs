

pub fn repeating_key_xor(bytes: &[u8], key: &[u8]) -> Vec<u8> {
    let cycled_key = key.iter().cycle();
    bytes.iter().zip(cycled_key.take(bytes.len()))
        .map(|tup| tup.0 ^ tup.1)
        .collect::<Vec<u8>>()
}
