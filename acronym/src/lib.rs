pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|c: char| !c.is_alphabetic() && c != '\'')
        .filter(|s| !s.is_empty())
        .map(|s| match s[1..].replace(char::is_uppercase, "").len() {
            0 => String::from(&s[..1]),
            _ => {
                String::from(&s[..1])
                    + &s[1..].replace(|c: char| !c.is_alphabetic() || c.is_lowercase(), "")
            }
        })
        .map(|j| j.to_uppercase())
        .collect()
}
