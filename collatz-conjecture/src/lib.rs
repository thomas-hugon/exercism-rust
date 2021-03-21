pub fn collatz(mut n: u64) -> Option<u64> {
    let mut count = 0;
    while n > 1 {
        count += 1;
        n = match n % 2 {
            0 => n / 2,
            _ => n * 3 + 1,
        }
    }
    Some(count).filter(|_| n == 1)
}
