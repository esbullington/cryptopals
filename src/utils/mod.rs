use data_encoding::BASE64;
use std::path::PathBuf;
use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;
use num;

pub fn return_raw(file: &str) -> Vec<u8> {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push(file);
    let file = File::open(d).unwrap();
    let buf = BufReader::new(file);
    let v: Vec<String> = buf.lines().map(|l| l.expect("Could not parse line")).collect();
    let s = v.concat();
    s.as_bytes().to_vec()
}

pub fn return_ciphertext(file: &str) -> Vec<u8> {
    let bytes = return_raw(file);
    BASE64.decode(&bytes).unwrap()
}

pub fn chunk_blocks(blocks: &[u8], k: usize) -> Vec<Vec<u8>> {
    let mut new_block: Vec<Vec<u8>> = Vec::new();
    for i in num::range_step(0, blocks.len(), k) {
        let block = &blocks[i..i+k];
        new_block.push(block.to_vec())
    }
    new_block
}
