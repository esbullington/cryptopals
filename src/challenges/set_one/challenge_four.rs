
use challenges::set_one::challenge_three::*;
use data_encoding::HEXLOWER;

pub fn find_sentence(file_contents: Vec<String>) -> String {
    let mut highest_score = 0usize;
    let mut highest_index = 0usize;
    for (i, s) in file_contents.iter().enumerate() {
        let bytes = HEXLOWER.decode(s.as_bytes()).unwrap();
        let score = score_byte_slice(&bytes);
        if score > highest_score {
            highest_score = score;
            highest_index = i;
        }
    }
    let correct_bytes = &file_contents[highest_index];
    let correct_dehexified = HEXLOWER.decode(correct_bytes.as_bytes()).unwrap();
    decode_ciphertext(&correct_dehexified)
}
