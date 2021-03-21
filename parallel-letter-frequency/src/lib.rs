use std::collections::HashMap;
use std::sync::Arc;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let input: Vec<String> = input.iter().map(|s| s.to_string()).collect();
    let len = input.len();
    let chunk_size = (input.len() / worker_count) + 1;
    let arc = Arc::new(input);
    let (tx, rx) = std::sync::mpsc::channel();
    let mut start = 0usize;
    for i in 0..worker_count {
        let arc = arc.clone();
        let tx = tx.clone();
        let end = ((i + 1) * chunk_size).min(len);

        // std::thread::spawn(move || {
        //     tx.send(seq_frequency(&arc[start..end])).unwrap();
        // });
        let closure = move || {
            tx.send(seq_frequency(&arc[start..end])).unwrap();
        };
        closure();
        start = end;
    }

    let mut merged = HashMap::new();
    for _ in 0..worker_count {
        let x = rx.recv().unwrap();
        x.into_iter()
            .for_each(|(k, v)| *(merged.entry(k).or_insert(0)) += v);
    }

    merged
}

pub fn seq_frequency(input: &[String]) -> HashMap<char, usize> {
    let mut map = HashMap::new();
    for line in input {
        for chr in line.chars().filter(|c| c.is_alphabetic()) {
            if let Some(c) = chr.to_lowercase().next() {
                (*map.entry(c).or_insert(0)) += 1;
            }
        }
    }

    map
}
