pub fn encrypt(input: &str) -> String {
    let normalized = normalize(input);
    if normalized.is_empty() {
        return String::from("");
    }
    let (c, r) = rectangle_dimensions(normalized.len());
    encrypt_rect(normalized, c, r)
        .chunks(r)
        .map(|c| c.iter().collect())
        .collect::<Vec<String>>()
        .join(" ")
}

fn rectangle_dimensions(len: usize) -> (usize, usize) {
    let sqrt = (len as f32).sqrt() as usize;
    let (c, r) = match sqrt * sqrt == len {
        true => (sqrt, sqrt),
        _ => (sqrt + 1, sqrt),
    };
    (c, r)
}

fn normalize(input: &str) -> Vec<char> {
    let normalized: Vec<_> = input
        .chars()
        .filter_map(|c| match c.is_ascii_digit() || c.is_ascii_alphabetic() {
            true => Some(c.to_ascii_lowercase()),
            _ => None,
        })
        .collect();
    normalized
}

fn encrypt_rect(normalized: Vec<char>, c: usize, r: usize) -> Vec<char> {
    let mut ret = Vec::new();
    let vec: Vec<Vec<char>> = normalized.chunks(c).map(|c| c.to_vec()).collect();
    for c in 0..c {
        for r in 0..r {
            ret.push(vec[r].get(c).map_or(' ', |c| *c));
        }
    }
    ret
}
