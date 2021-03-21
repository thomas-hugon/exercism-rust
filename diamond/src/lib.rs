pub fn get_diamond(c: char) -> Vec<String> {
    let c = c.to_ascii_uppercase();
    let len = c as usize - b'A' as usize + 1;

    let half_vec: Vec<_> = std::iter::once(
            format!("{0}{1}{0}", " ".repeat(len - 1), 'A')
        )
        .chain(('A'..=c).enumerate().skip(1).map(|(i, c)|
            format!("{0}{1}{2}{1}{0}"," ".repeat(len - i - 1),c," ".repeat(i * 2 - 1))
        ))
        .collect();

    half_vec.iter().chain(half_vec.iter().rev().skip(1)).cloned().collect()
}
