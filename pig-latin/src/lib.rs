pub fn translate(input: &str) -> String {
    input
        .split_ascii_whitespace()
        .map(|s| translate_word(s))
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn translate_word(input: &str) -> String {
    const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
    const VOWELS_AND_Y: [char; 6] = ['a', 'e', 'i', 'o', 'u', 'y'];

    if input.starts_with(&VOWELS[..]) || input.starts_with("xr") || input.starts_with("yt") {
        input.to_string() + "ay"
    } else {
        input
            .find('y')
            .filter(|i| i == &0)
            .map(|_| 1)
            .or_else(|| {
                input.find("qu").and_then(|i1| {
                    input
                        .find(&VOWELS_AND_Y[..])
                        .map(|i2| if i1 < i2 { i1 + 2 } else { i2 })
                })
            })
            .or_else(|| input.find(&VOWELS_AND_Y[..]))
            .map(|i| input.split_at(i))
            .map_or_else(|| input.to_string(), |(f, s)| s.to_string() + f)
            + "ay"
    }
}
