extern crate cryptopals;

use cryptopals::challenges::set_two::*;

#[test]
fn test_challenge_one() {
    let unpadded = b"YELLOW SUBMARINE";
    let padded = challenge_nine::pad_bytes(unpadded, 20, 0x04);
    assert_eq!("YELLOW SUBMARINE\x04\x04\x04\x04", String::from_utf8(padded).unwrap());
}
