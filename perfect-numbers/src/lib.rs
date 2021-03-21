use crate::Classification::{Abundant, Deficient, Perfect};

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    match (num, (1..=num / 2).filter(|&x| num % x == 0).sum::<u64>()) {
        (0, _) => None,
        (a, b) if a == b => Some(Perfect),
        (a, b) if a < b => Some(Abundant),
        _ => Some(Deficient),
    }
}
