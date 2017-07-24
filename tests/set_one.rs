extern crate cryptopals;
extern crate data_encoding;

use cryptopals::challenges::set_one::*;
use data_encoding::HEXLOWER;
use data_encoding::BASE64;
use std::path::PathBuf;
use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;


#[test]
fn challenge_one() {
    let hex_string = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let base_64_string = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t".to_owned();
    assert_eq!(base_64_string, challenge_one::hex_to_base64(&hex_string));
}


#[test]
fn challenge_two() {
    let original_string = "1c0111001f010100061a024b53535009181c";
    let xor_string = "686974207468652062756c6c277320657965";
    let result_string = "746865206b696420646f6e277420706c6179";
    assert_eq!(result_string, challenge_two::fixed_xor(&original_string, &xor_string));
}

#[test]
fn challenge_three() {
    let ciphertext = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let bytes = HEXLOWER.decode(ciphertext.as_bytes()).unwrap();
    let plaintext = challenge_three::decode_ciphertext(&bytes);
    assert_eq!("Cooking MC's like a pound of bacon", plaintext);
}

#[test]
fn challenge_four() {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("resources/challenges/set_one/4.txt");
    let file = File::open(d).unwrap();
    let buf = BufReader::new(file);
    let v = buf.lines().map(|l| l.expect("Could not parse line")).collect();
    let sentence = challenge_four::find_sentence(v);
    assert_eq!("Now that the party is jumping", sentence.trim());
}

#[test]
fn challenge_five() {
    let plaintext = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal".as_bytes();
    let ciphertext = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";
    let key = "ICE".as_bytes();
    let result = challenge_five::repeating_key_xor(plaintext, key);
    let hexlified_result = HEXLOWER.encode(&result);
    assert_eq!(ciphertext, hexlified_result);
}

fn return_ciphertext(file: &str) -> Vec<u8> {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("resources/challenges/set_one/");
    d.push(file);
    let file = File::open(d).unwrap();
    let buf = BufReader::new(file);
    let v: Vec<String> = buf.lines().map(|l| l.expect("Could not parse line")).collect();
    let s = v.concat();

    let bytes = s.as_bytes();
    BASE64.decode(&bytes).unwrap()
}

#[test]
fn challenge_six() {
    let file = "6.txt";
    let ciphertext = return_ciphertext(file);
    let solution = challenge_six::search_for_solution(&ciphertext);
    assert_eq!("I'm back and I'm ringin' the bell", &solution[0..33]);
}

#[test]
fn challenge_seven() {
    let file = "7.txt";
    let ciphertext = return_ciphertext(file);
    let key = "YELLOW SUBMARINE".as_bytes();
    let decrypted = challenge_seven::decrypt(&ciphertext, &key).unwrap();
    let plaintext = String::from_utf8(decrypted).unwrap();
    assert_eq!("I'm back and I'm ringin' the bell", &plaintext[0..33]);
}
