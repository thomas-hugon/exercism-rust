use rand::*;
pub fn encode(key: &str, s: &str) -> Option<String> {
    if key.is_empty() || key.chars().any(|c| !c.is_ascii_lowercase()) {
        return None;
    }
    Some(
        s.chars()
            .zip(key.chars().cycle())
            .map(|(c, k)| ((c as u8 - b'a' + k as u8 - b'a') % 26 + b'a') as char)
            .collect(),
    )
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    if key.is_empty() || key.chars().any(|c| !c.is_ascii_lowercase()) {
        return None;
    }
    Some(
        s.chars()
            .zip(key.chars().cycle())
            .map(|(c, k)| (b'a' + (26 - ((k as u8 + 26 - c as u8) % 26) as u8) % 26) as char)
            .collect(),
    )
}

pub fn encode_random(s: &str) -> (String, String) {
    let key: String = (0..thread_rng().next_u32() % 50 + 100)
        .map(|_| (thread_rng().gen::<u8>() % 26 + b'a') as char)
        .collect();
    let encoded = encode(&key, s);
    (key, encoded.unwrap())
}
