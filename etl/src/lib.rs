use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    h.iter()
        .flat_map(|(&s, ls)| ls.iter().map(move |&l| (l.to_ascii_lowercase(), s)))
        .collect()
}
