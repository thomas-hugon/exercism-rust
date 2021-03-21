use crate::AffineCipherError::NotCoprime;

/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if !coprime(a as u32, 26) {
        return Err(NotCoprime(a));
    }
    Ok(plaintext
        .chars()
        .filter(|c| c.is_ascii_digit() || c.is_ascii_alphabetic())
        .map(|c| from_letter_index(encode_char(a, b, to_letter_index(c))))
        .collect::<Vec<char>>()
        .chunks(5)
        .map(|w| w.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join(" "))
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if !coprime(a as u32, 26) {
        return Err(NotCoprime(a));
    }
    let mmi = mmi(a as u32, 26) as i32;
    Ok(ciphertext
        .chars()
        .filter(|c| c.is_ascii_digit() || c.is_ascii_alphabetic())
        .map(|c| from_letter_index(decode_char(mmi, b, to_letter_index(c))))
        .collect())
}

fn encode_char(a: i32, b: i32, x: i32) -> i32 {
    match x >= 0 {
        false => x,
        _ => (a as i32 * x + b) % 26,
    }
}

fn decode_char(mmi: i32, b: i32, y: i32) -> i32 {
    match y >= 0 {
        false => y,
        _ => ((y + b * 25) * mmi) % 26,
    }
}

fn to_letter_index(c: char) -> i32 {
    c.to_ascii_lowercase() as i32 - b'a' as i32
}

fn from_letter_index(i: i32) -> char {
    (i + b'a' as i32) as u8 as char
}

fn mmi(a: u32, m: u32) -> u32 {
    for i in 0..(a * m) {
        if a * i % m == 1 {
            return i;
        }
    }
    panic!("mmi does not exist for {},{} ", a, m);
}

fn coprime(mut m: u32, mut n: u32) -> bool {
    while m != 0 {
        let temp = m;
        m = n % temp;
        n = temp;
    }
    n == 1
}
