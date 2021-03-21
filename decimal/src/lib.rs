use num_bigint::BigInt;
use std::cmp::Ordering;
use std::fmt::{Debug};
use std::ops::{Add, Mul, Sub};
use std::str::FromStr;

/// Type implementing arbitrary-precision decimal arithmetic
#[derive(Debug)]
pub struct Decimal {
    unscaled_value: BigInt,
    scale: u32,
}

impl Decimal {
    pub fn try_from(input: &str) -> Option<Decimal> {
        BigInt::from_str(&input.replacen('.', "", 1))
            .map(|int| Decimal {
                unscaled_value: int,
                scale: input
                    .rfind('.')
                    .filter(|i| *i < input.len() - 1)
                    .map(|i| input.len() - 1 - i)
                    .unwrap_or(0) as u32,
            })
            .ok()
    }

    pub fn scale(&self, scale: u32) -> Decimal {
        if scale < self.scale {
            Decimal {
                unscaled_value: (0..(self.scale - scale) / 18)
                    .map(|_| 1_000_000_000_000_000_000u64)
                    .chain(std::iter::once(10u64.pow((self.scale - scale) % 18)))
                    .fold(self.unscaled_value.clone(), |acc, m| acc / m),
                scale,
            }
        } else {
            Decimal {
                unscaled_value: (0..(scale - self.scale) / 18)
                    .map(|_| 1_000_000_000_000_000_000u64)
                    .chain(std::iter::once(10u64.pow((scale - self.scale) % 18)))
                    .fold(self.unscaled_value.clone(), |acc, m| acc * m),
                scale,
            }
        }
    }
}

impl PartialEq for Decimal {
    fn eq(&self, other: &Self) -> bool {
        let scale = self.scale.max(other.scale);
        self.scale(scale).unscaled_value == other.scale(scale).unscaled_value
    }
}
impl Eq for Decimal {}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let scale = self.scale.max(other.scale);
        self.scale(scale).unscaled_value.partial_cmp(&other.scale(scale).unscaled_value)
    }
}
impl Ord for Decimal {
    fn cmp(&self, other: &Self) -> Ordering {
        let scale = self.scale.max(other.scale);
        self.scale(scale).unscaled_value.cmp(&other.scale(scale).unscaled_value)
    }
}
impl Add for Decimal {
    type Output = Decimal;

    fn add(self, other: Self) -> Self::Output {
        let scale = self.scale.max(other.scale);
        Decimal {
            unscaled_value: self.scale(scale).unscaled_value + other.scale(scale).unscaled_value,
            scale,
        }
    }
}
impl Sub for Decimal {
    type Output = Decimal;

    fn sub(self, other: Self) -> Self::Output {
        let scale = self.scale.max(other.scale);
        Decimal {
            unscaled_value: self.scale(scale).unscaled_value - other.scale(scale).unscaled_value,
            scale,
        }
    }
}
impl Mul for Decimal {
    type Output = Decimal;

    fn mul(self, other: Self) -> Self::Output {
        Decimal {
            unscaled_value: self.unscaled_value * other.unscaled_value,
            scale: self.scale + other.scale,
        }
    }
}
