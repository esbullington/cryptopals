

pub fn fixed_xor(original_string: &[u8], xor_string: &[u8]) -> Vec<u8> {
    let mut xored = Vec::new();
    for (x, y) in original_string.iter().zip(xor_string) {
        xored.push(x^y);
    }
    xored
}
