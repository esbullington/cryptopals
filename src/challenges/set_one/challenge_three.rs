
use data_encoding::HEXLOWER;


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

fn score_byte_vector(ciphertext: &Vec<u8>) -> usize {
    ciphertext
        .iter()
        .fold(0usize, |acc, val| acc + score_byte(*val))
}

fn xor_byte_vector(ciphertext: &Vec<u8>, byte: u8) -> Vec<u8> {
    ciphertext.iter().map(|&t| t ^ byte ).collect()
}

pub fn decode_ciphertext(ciphertext: &str) -> String {
    let bytes = HEXLOWER.decode(ciphertext.as_bytes()).unwrap();
    let mut top_score_xored: Vec<u8> = Vec::new();
    let mut top_score = 0usize;
    for byte in 0...255 {
        let xored = xor_byte_vector(&bytes, byte as u8);
        let scored = score_byte_vector(&xored);
        if scored > top_score {
            top_score = scored;
            top_score_xored = xored;
        }
    }
    String::from_utf8(top_score_xored).unwrap()
}

#[test]
fn challenge_three() {
    let ciphertext = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let plaintext = decode_ciphertext(&ciphertext);
    assert_eq!("Cooking MC's like a pound of bacon", plaintext);
}
