
use data_encoding::BASE64;
use std::path::PathBuf;
use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;

pub fn return_ciphertext(file: &str) -> Vec<u8> {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push(file);
    let file = File::open(d).unwrap();
    let buf = BufReader::new(file);
    let v: Vec<String> = buf.lines().map(|l| l.expect("Could not parse line")).collect();
    let s = v.concat();

    let bytes = s.as_bytes();
    BASE64.decode(&bytes).unwrap()
}

