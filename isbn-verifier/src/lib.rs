/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    isbn.chars()
        .filter(|c| c != &'-')
        .enumerate()
        .try_fold((0, 0), |(s, cnt), (i, c)| match (10 - i, c.is_digit(10)) {
            (1, false) if c == 'X' => Some((s + 10, cnt + 1)),
            (i, true) => Some((s + i as u32 * c.to_digit(10).unwrap(), cnt + 1)),
            _ => None,
        })
        .map_or(false, |(sum, cn)| cn == 10 && sum % 11 == 0)
}
