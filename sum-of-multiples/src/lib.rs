use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let uniques: HashSet<u32> = factors
        .iter()
        .filter(|&x| x != &0 && x < &limit)
        .flat_map(|x| (1..((limit + x - 1) / x)).map(move |y| x * y))
        .collect();

    uniques.iter().sum()
}
