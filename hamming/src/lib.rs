/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    Some(s1.len() == s2.len()).filter(|&x| x).map(|_| {
        s1.chars()
            .zip(s2.chars())
            .filter(|(s1, s2)| s1 != s2)
            .count()
    })
}
