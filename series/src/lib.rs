pub fn series(digits: &str, len: usize) -> Vec<String> {
    (0..((digits.len() as i32 - len as i32 + 1).max(0) as usize))
        .map(|i| digits[i..i + len].to_string())
        .collect()
}
