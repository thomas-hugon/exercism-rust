pub fn rotate(input: &str, key: i8) -> String {
    input
        .chars()
        .map(|c| match c {
            'a'..='z' => ((c as i8 - b'a' as i8 + key).rem_euclid(26) as u8 + b'a') as char,
            'A'..='Z' => ((c as i8 - b'A' as i8 + key).rem_euclid(26) as u8 + b'A') as char,
            _ => c,
        })
        .collect()
}
