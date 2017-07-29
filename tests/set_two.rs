extern crate cryptopals;

use cryptopals::challenges::set_two::*;

#[test]
fn test_challenge_one() {
    let unpadded = b"YELLOW SUBMARINE";
    let padded = challenge_nine::pad_bytes(unpadded, 20, 0x04);
    assert_eq!("YELLOW SUBMARINE\x04\x04\x04\x04", String::from_utf8(padded).unwrap());
}

#[test]
fn challenge_ten() {
    let file = "resources/challenges/set_two/10.txt";
    let ciphertext = cryptopals::utils::return_ciphertext(file);
    let key = "YELLOW SUBMARINE".as_bytes();
    let iv = vec![0x00;16];
    let decrypted = challenge_ten::decrypt_cbc(&ciphertext, &key, &iv).unwrap();
    let plaintext = String::from_utf8(decrypted).unwrap();
    assert_eq!("I'm back and I'm ringin' the bell", &plaintext[0..33]);
}

