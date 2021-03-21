pub fn nth(n: u32) -> u32 {
    let mut primes: Vec<u32> = Vec::with_capacity((n + 1) as usize);
    primes.push(2);
    for i in 3.. {
        if primes.len() == (n + 1) as usize {
            return *primes.last().unwrap();
        }
        if is_prime(i, &primes[0..primes.len() / 2 + 1]) {
            primes.push(i)
        }
    }
    panic!()
}

fn is_prime(nb: u32, primes: &[u32]) -> bool {
    !primes.iter().any(|x| nb % x == 0)
}
