use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    words.split(|c:char| !c.is_ascii_alphanumeric() && c != '\'' )
        .map(|word| word.trim_matches('\'').to_lowercase())
        .filter(|s|!s.is_empty())
        .fold(HashMap::new(), |mut acc,word| {
            *acc.entry(word.to_string()).or_insert(0) += 1;
            acc
        })
}
