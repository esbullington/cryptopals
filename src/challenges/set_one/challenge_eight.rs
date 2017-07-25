
use itertools::Itertools;
use num;

fn chunk_blocks(blocks: &[u8], k: usize) -> Vec<Vec<u8>> {
    let mut new_block: Vec<Vec<u8>> = Vec::new();
    for i in num::range_step(0, blocks.len(), k) {
        let block = &blocks[i..i+k];
        new_block.push(block.to_vec())
    }
    new_block
}

fn find_score(nested_blocks: Vec<Vec<u8>>) -> usize {
    let mut count = 0usize;
    for (i, j) in nested_blocks.iter().tuple_combinations() {
        if i == j {
            count += 1;
        }
    }
    count
}

pub fn find_aes_ecb_block(ciphertext: Vec<Vec<u8>>) -> usize {
    let keysize = 16usize;
    let mut line = 0usize;
    for (i, block) in ciphertext.iter().enumerate() {
        let chunked = chunk_blocks(block, keysize);
        let score = find_score(chunked);
        if score > 0 {
            line = i;
        }
    }
    line
}
