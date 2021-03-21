use regex::Regex;

pub fn number(user_number: &str) -> Option<String> {
    Regex::new(r"^\D*1?\D*([2-9]\d{2})\D*([2-9]\d{2})\D*(\d{4})\D*$").ok()?
        .captures(user_number)
        .map(|captures| format!("{}{}{}", &captures[1], &captures[2], &captures[3]))
}
