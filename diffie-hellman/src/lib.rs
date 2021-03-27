use rand::Rng;

pub fn private_key(p: u64) -> u64 {
    rand::thread_rng().gen_range(1..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    mod_pow(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    mod_pow(b_pub, a, p)
}

fn mod_pow(base: u64, mut exp: u64, m: u64) -> u64 {
    let mut base = base as u128;
    let m = m as u128;
    let mut result = 1;
    while exp > 0 {
        if (exp & 1) > 0 {
            result = (result * base) % m;
        }
        exp >>= 1;
        base = (base * base) % m;
    }

    result as u64
}
