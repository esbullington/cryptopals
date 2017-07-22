extern crate cryptopals;
extern crate data_encoding;

use cryptopals::challenges::set_one::*;
use data_encoding::HEXLOWER;
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
    println!("{}", sentence);
    assert_eq!("Now that the party is jumping", sentence.trim());
}
