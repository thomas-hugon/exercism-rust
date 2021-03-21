/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    code.chars()
        .rev()
        .filter(|c| !c.is_whitespace())
        .enumerate()
        .map(|(i, c)| ( i,
                c.to_digit(10)
                    .map(|val| if i % 2 == 1 { val * 2 } else { val })
                    .map(|val| if val > 9 { val - 9 } else { val }),
            )
        )
        .try_fold((0, 0), |(_, sum), (i, c)| {
            c.map(|val| (i, val + sum))
        })
        .map_or(false, |(last_index, sum)| sum % 10 == 0 && last_index > 0)
}
