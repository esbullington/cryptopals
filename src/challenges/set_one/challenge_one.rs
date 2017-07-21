
use data_encoding::HEXLOWER;
use data_encoding::BASE64;

pub fn hex_to_base64(hex_string: &str) -> String {
    let bytes = hex_string.as_bytes();
    let decoded = HEXLOWER.decode(bytes).unwrap();
    BASE64.encode(&decoded)
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

}
