
use data_encoding::HEXLOWER;
use data_encoding::BASE64;

pub fn hex_to_base64(hex_string: &str) -> String {
    let bytes = hex_string.as_bytes();
    let decoded = HEXLOWER.decode(bytes).unwrap();
    BASE64.encode(&decoded)
}

pub fn fixed_xor(original_string: &str, xor_string: &str) -> String {
    let mut xored = Vec::new();
    let xs = HEXLOWER.decode(original_string.as_bytes()).unwrap();
    let ys = HEXLOWER.decode(xor_string.as_bytes()).unwrap();
    for (x, y) in xs.iter().zip(ys) {
        xored.push(x^y);
    }
    HEXLOWER.encode(&xored)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn challenge_one() {
        let hex_string = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let base_64_string = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t".to_owned();
        assert_eq!(base_64_string, hex_to_base64(&hex_string));
    }

    #[test]
    fn challenge_two() {
        let original_string = "1c0111001f010100061a024b53535009181c";
        let xor_string = "686974207468652062756c6c277320657965";
        let result_string = "746865206b696420646f6e277420706c6179";
        assert_eq!(result_string, fixed_xor(&original_string, &xor_string));
    }

}
