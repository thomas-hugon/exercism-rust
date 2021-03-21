use std::collections::HashSet;

type Norm = (String, Vec<char>);

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word = normalize(word);

    possible_anagrams
        .iter()
        .filter(|&&s| is_anagram(&word, &normalize(s)))
        .map(|&s| s)
        .collect()
}

fn normalize(word: &str) -> Norm {
    let normalized_word = word.to_lowercase();
    let mut word_chars_sorted: Vec<char> = normalized_word.chars().collect();
    word_chars_sorted.sort_unstable();
    (normalized_word, word_chars_sorted)
}

fn is_anagram(left: &Norm, right: &Norm) -> bool {
    left.0 != right.0 && left.1 == right.1
}
