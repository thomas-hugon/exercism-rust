use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

pub fn check(candidate: &str) -> bool {
    let candidate = candidate.replace(|c: char| c == '-' || c.is_whitespace(), "");
    candidate.len() == candidate.to_lowercase().chars().collect::<HashSet<char>>().len()
}
