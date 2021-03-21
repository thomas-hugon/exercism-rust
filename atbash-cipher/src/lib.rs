/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    decode(plain)
        .chars().collect::<Vec<char>>()
        .chunks(5).map(|c| c.iter().collect()).collect::<Vec<String>>()
        .join(" ")
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher
        .chars()
        .filter_map(|c| match (c.is_ascii_digit(), c.is_ascii_alphabetic()) {
            (true, _) => Some(c),
            (_, true) => Some((b'z' - c.to_ascii_lowercase() as u8 + b'a') as char),
            _ => None,
        })
        .collect()
}
