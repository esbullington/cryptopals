
use data_encoding::HEXLOWER;

pub fn fixed_xor(original_string: &str, xor_string: &str) -> String {
    let mut xored = Vec::new();
    let xs = HEXLOWER.decode(original_string.as_bytes()).unwrap();
    let ys = HEXLOWER.decode(xor_string.as_bytes()).unwrap();
    for (x, y) in xs.iter().zip(ys) {
        xored.push(x^y);
    }
    HEXLOWER.encode(&xored)
}


#[test]
fn challenge_two() {
    let original_string = "1c0111001f010100061a024b53535009181c";
    let xor_string = "686974207468652062756c6c277320657965";
    let result_string = "746865206b696420646f6e277420706c6179";
    assert_eq!(result_string, fixed_xor(&original_string, &xor_string));
}
