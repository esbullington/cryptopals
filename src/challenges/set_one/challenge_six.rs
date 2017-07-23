
use challenges::set_one::challenge_three::*;

// Loop through byte, shifting each bit to the first position,
// and masking off the remaining bits, counting as we go
fn count_bits(byte: u8) -> usize {
    (0..8).fold(0usize, |acc, i| {
         acc + ((byte >> i) & 0x01) as usize
    })
}

pub fn hamming_score(one: &[u8], two: &[u8]) -> usize {
    one.iter().zip(two)
        .map(|tup| tup.0 ^ tup.1)
        .fold(0usize, |acc, val| acc + count_bits(val))
}

// vec!(0,1,2,3) to vec!(vec!(0, 2),vec!(1, 3)) for key of 2
fn split_apart(ciphertext: &[u8], key: u8) -> Vec<Vec<u8>> {
    let mut ret: Vec<Vec<u8>> = Vec::new();
    for i in 0..key {
        ret.push(Vec::new());
    }
    let cycled = (0..key).cycle();
    for (i, val) in cycled.zip(ciphertext) {
        ret[i as usize].push(*val);
    }
    ret
}

// vec!(vec!(0, 2),vec!(1, 3)) to vec!(0,1,2,3)
fn merge_together(split_text: Vec<Vec<u8>>) -> Vec<u8> {
    let mut ret = Vec::new();
    let m = split_text[0].len();
    let n = split_text.len();
    for i in 0..m {
        for j in 0..n {
            let item = split_text[j][i];
            ret.push(item);
        }
    }
    ret
}

// fn find_with_key(ciphertext: &[u8], key: u8) -> Vec<u8> {
// }

pub fn find_keysize(ciphertext: &[u8]) -> Vec<usize> {
    // let for_use = (2..40).collect::<Vec<usize>>()
    //     });
    vec![1usize]
}

#[cfg(test)]
mod tests {

    use super::*;

    use std::path::PathBuf;
    use std::io::BufReader;
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn test_hamming_score() {
        let one = b"this is a test";
        let two = b"wokka wokka!!!";
        assert_eq!(37, hamming_score(one, two));
        // assert_eq!(true, true);
    }

    #[test]
    fn test_merge_together() {
        assert_eq!(
            merge_together(vec!(vec!(0,3), vec!(1,4), vec!(2,5))),
            vec!(0,1,2,3,4,5)
            );
    }

    #[test]
    fn test_split_apart() {
        let test_vec = vec!(0,1,2,3,4,5);
        assert_eq!(
            split_apart(&test_vec, 3u8),
            vec!(vec!(0,3), vec!(1,4), vec!(2,5))
            );
    }

    // #[test]
    // fn test_find_keysize() {
    //     let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    //     d.push("resources/challenges/set_one/6.txt");
    //     let file = File::open(d).unwrap();
    //     let buf = BufReader::new(file);
    //     let v: Vec<String> = buf.lines().map(|l| l.expect("Could not parse line")).collect();
    //     let s: String = v.concat();
    //     let plaintext = s.as_bytes();
    //     find_keysize(plaintext);
    //     }
}
