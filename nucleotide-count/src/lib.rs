use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    nucleotide_counts(dna)?
        .remove(&nucleotide)
        .ok_or(nucleotide)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    dna.chars().try_fold(empty_map(), |mut acc, c| {
        acc.get_mut(&c)
            .map(|count| *count += 1)
            .map(|_| acc)
            .ok_or(c)
    })
}

fn empty_map() -> HashMap<char, usize> {
    let mut map = HashMap::new();
    map.insert('A', 0);
    map.insert('G', 0);
    map.insert('C', 0);
    map.insert('T', 0);
    map
}
