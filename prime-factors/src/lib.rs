pub fn factors(n: u64) -> Vec<u64> {
    let mut n = n;
    let mut factors = Vec::<u64>::new();
    let mut i = 2;

    while n > 1 {
        while n % i == 0 {
            n /= i;
            factors.push(i);
        }
        i += 1;
    }

    factors
}
