use crypto::{ symmetriccipher };
use challenges::set_one::*;
use num;

pub fn decrypt_cbc(ciphertext: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    let keysize = key.len();
    let mut results: Vec<u8> = Vec::new();
    let mut to_xor = iv.to_vec();
	for i in num::range_step(0, ciphertext.len(), keysize) {
		let block = &ciphertext[i..i + keysize];
        let decrypted = challenge_seven::decrypt(&block, &key).unwrap();
        let xored = challenge_two::fixed_xor(&decrypted, &to_xor);
        results.extend(&xored);
        to_xor = block.to_vec().clone();
	}
    Ok(results)
}
