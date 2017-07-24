
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
fn break_blocks_apart(ciphertext: &[u8], key: u8) -> Vec<Vec<u8>> {
    let mut ret: Vec<Vec<u8>> = (0..key).fold(Vec::new(),|mut acc, _| {
        acc.push(Vec::new());
        acc
    });
    let cycled = (0..key).cycle();
    for (i, val) in cycled.zip(ciphertext) {
        ret[i as usize].push(*val);
    }
    ret
}

// vec!(vec!(0, 2),vec!(1, 3)) to vec!(0,1,2,3)
fn merge_blocks_together(split_text: Vec<Vec<u8>>) -> Vec<u8> {
    let length = split_text.len();
    (0..).take_while( |&i| {
        let modulus_vector = &split_text[i % length];
        modulus_vector.get(i / length).is_some()
    }).fold(Vec::new(), |mut acc, i| {
        let modulus_vector = &split_text[i % length];
        let val = &modulus_vector[i / length];
        acc.push(*val);
        acc
    })
}

fn find_with_keysize(ciphertext: &[u8], keysize: u8) -> Vec<u8> {
    let xss = break_blocks_apart(ciphertext, keysize);
    let mut keys: Vec<u8> = Vec::new();
    for xs in &xss {
        let found_key = find_key(&xs);
        keys.push(found_key);
    }
    let mut results = Vec::new();
    for (ys, key) in xss.iter().zip(keys.iter()) {
        let decoded: Vec<u8> = ys.iter().map(|x| x ^ key).collect();
        results.push(decoded);
    }
    merge_blocks_together(results)
}

pub fn search_for_solution(ciphertext: &[u8]) -> String {
    let mut high_score = 0usize;
    let mut best_solution: Vec<u8> = Vec::new();    
    for keysize in 2..40 {
        let uk = keysize as u8;
        let possible_solution = find_with_keysize(ciphertext, uk);
        let score = score_byte_slice(&possible_solution);
        if score > high_score {
            high_score = score;
            best_solution = possible_solution;
        } 
    }
    String::from_utf8(best_solution).unwrap()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_hamming_score() {
        let one = b"this is a test";
        let two = b"wokka wokka!!!";
        assert_eq!(37, hamming_score(one, two));
    }

    #[test]
    fn test_merge_blocks_together() {
        assert_eq!(
            merge_blocks_together(vec!(vec!(0,3), vec!(1,4), vec!(2,5))),
            vec!(0,1,2,3,4,5)
            );
    }

    #[test]
    fn test_break_blocks_apart() {
        let test_vec = vec!(0,1,2,3,4,5);
        assert_eq!(
            break_blocks_apart(&test_vec, 3u8),
            vec!(vec!(0,3), vec!(1,4), vec!(2,5))
            );
    }

}
