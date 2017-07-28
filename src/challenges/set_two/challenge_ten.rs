use crypto::{ symmetriccipher, buffer, aes, blockmodes };
use crypto::buffer::{ ReadBuffer, WriteBuffer, BufferResult };

pub fn encrypt(plaintext: &[u8], key: &[u8]) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    let mut encryptor = aes::ecb_encryptor(
            aes::KeySize::KeySize128,
            key,
            blockmodes::PkcsPadding);

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(plaintext);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = try!(encryptor.encrypt(&mut read_buffer, &mut write_buffer, true));
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => { }
        }
    }

    Ok(final_result)
}

pub fn encrypt_cbc(plaintext: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
}

#[cfg(test)]
mod tests {

    use super::*;
    use utils::*;
    use data_encoding::BASE64;

    #[test]
    fn challenge_two() {
        let file = "resources/challenges/set_two/10.txt";
        let benchmark_ciphertext = return_ciphertext(file);
        let key = "YELLOW SUBMARINE".as_bytes();
        let iv = b"0x00000";
        let ciphertext = encrypt_cbc(&plaintext, &key).unwrap();
        let test_ciphertext = BASE64.encode(&ciphertext).unwrap();
        assert_eq!(benchmark_ciphertext, test_ciphertext);
    }


}
