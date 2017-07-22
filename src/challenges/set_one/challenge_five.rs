
use data_encoding::HEXLOWER;


pub fn repeating_key_xor(bytes: &[u8], key: &[u8]) -> String {
    let cycled_key = key.iter().cycle();
    let output = bytes.iter().zip(cycled_key.take(bytes.len()))
        .map(|tup| tup.0 ^ tup.1)
        .collect::<Vec<u8>>();
    HEXLOWER.encode(&output)
}
