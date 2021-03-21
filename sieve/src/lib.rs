pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut vec: Vec<Option<u64>> = (2..=upper_bound).map(|i| Option::from(i)).collect();
    let mut primes = Vec::new();
    for i in 0..vec.len() {
        if (vec[i].is_some()) {
            vec[i]
        }
    }

    for i in prime..(upper_bound / prime) + 1 {
        if let Some(i) = vec.iter().position(|&x| x == i * prime) {
            vec.remove(i);
        }
    }
    primes.push(prime);

    primes
}
pub fn primes_up_to2(upper_bound: u64) -> Vec<u64> {
    let mut vec: Vec<u64> = (2..=upper_bound).rev().collect();
    let mut primes = Vec::new();
    while let Some(prime) = vec.pop() {
        for i in prime..(upper_bound / prime) + 1 {
            if let Some(i) = vec.iter().position(|&x| x == i * prime) {
                vec.remove(i);
            }
        }
        primes.push(prime);
    }

    primes
}
