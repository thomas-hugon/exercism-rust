use std::option::Option::Some;

pub fn encode(source: &str) -> String {
    source
        .chars()
        .fold(Vec::new(), |mut acc: Vec<(usize, char)>, c| {
            match acc.last_mut() {
                Some((cnt, lc)) if lc == &c => *cnt += 1,
                _ => acc.push((1, c)),
            }
            acc
        })
        .iter()
        .map(|(cnt, c)| match cnt {
            1 => c.to_string(),
            n => format!("{}{}", n, c),
        })
        .collect()
}

pub fn decode(source: &str) -> String {
    fn decode(count: usize, c: char) -> String {
        match count {
            0..=1 => c.to_string(),
            _ => c.to_string().repeat(count),
        }
    }
    source
        .chars()
        .fold((0, String::new()), |(cnt, s), c| match c.is_digit(10) {
            false => (0, s + &decode(cnt, c)),
            true => (cnt * 10 + c.to_digit(10).unwrap() as usize, s),
        })
        .1
}
