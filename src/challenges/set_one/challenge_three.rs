

fn score_byte(c: u8) -> usize {
    let normalized_char = match c {
        b'A'...b'Z' => {
            let relative_offset = c - b'A';
            relative_offset + b'a'
        }
        _ => c
    };
    match normalized_char {
        b'e' => 5,
        b'a' | b't' => 4,
        b'h' | b'i' | b'n' | b'o' | b'r' | b's' | b' ' => 3,
        b'b' | b'c' | b'd' | b'f' | b'g' | b'k' | b'l' | b'm' | b'p'
             | b'u' | b'v' | b'w' | b'y' => 2,
        b'j' | b'q' | b'x' | b'z' => 1,
        _ => 0,
    }
}

pub fn score_byte_slice(cipherbytes: &[u8]) -> usize {
    cipherbytes
        .iter()
        .fold(0usize, |acc, val| acc + score_byte(*val))
}

fn xor_byte_slice(cipherbytes: &[u8], byte: u8) -> Vec<u8> {
    cipherbytes.iter().map(|&t| t ^ byte ).collect()
}

pub fn find_key(bytes: &[u8]) -> u8 {
    (0...255).max_by_key(|key| {
        let xored = xor_byte_slice(&bytes, *key as u8);
        score_byte_slice(&xored)
    }).unwrap()
}

pub fn decode_ciphertext(bytes: &[u8]) -> String {
    let key = find_key(&bytes);
    let key_xored = xor_byte_slice(&bytes, key);
    String::from_utf8(key_xored).unwrap()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_score_byte_slice() {
        let bytes = b"ETAOIN SHRDLU";
        assert_eq!(40, score_byte_slice(bytes));
    }

    #[test]
    fn test_find_key() {
        let bytes = b"testing";
        let key = 42u8;
        let xored: Vec<u8> = xor_byte_slice(bytes, key);
        assert_eq!(find_key(&xored), key);
    }

}

