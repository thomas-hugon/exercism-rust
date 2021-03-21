#[derive(Debug, PartialEq, Eq)]
pub struct Palindrome {
    pal: u64,
}

impl Palindrome {
    pub fn new(a: u64, b: u64) -> Palindrome {
        Self::from(a * b)
    }

    pub fn from(pal: u64) -> Palindrome {
        Palindrome { pal }
    }

    pub fn value(&self) -> u64 {
        self.pal
    }

    pub fn insert(&mut self, a: u64, b: u64) {
        assert_eq!(self.pal, a * b)
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    (min..=max)
        .flat_map(|x| {
            (min..=max).filter_map(move |y| match x * y {
                a if is_palindrome(a) => Some(a),
                _ => None,
            })
        })
        .fold(None, |acc, pal| match (acc, pal) {
            (None, pal) => Some((pal, pal)),
            (Some((min, max)), pal) if pal < min => Some((pal, max)),
            (Some((min, max)), pal) if pal > max => Some((min, pal)),
            _ => acc,
        })
        .map(|(min, max)| (Palindrome::from(min), Palindrome::from(max)))
}

fn is_palindrome(a: u64) -> bool {
    let mut b = a;
    let mut rev = 0;
    while b > 0 {
        rev = rev * 10 + b % 10;
        b /= 10;
    }
    a == rev
}
