use crate::TriangleType::{Equilateral, Isosceles, Scalene};
use std::cmp::Ordering::Greater;
use std::ops::Add;

pub struct Triangle(TriangleType);
#[derive(PartialEq)]
enum TriangleType {
    Equilateral,
    Isosceles,
    Scalene,
}

pub trait SuperiorZero: Copy + PartialOrd + PartialEq + Add<Output = Self> {
    fn is_strictly_superior_to_zero(&self) -> bool;
}
macro_rules! impl_superior_zero {
    ($($t:ty)*) => ($(
        impl SuperiorZero for $t {
            fn is_strictly_superior_to_zero(&self) -> bool {
                self > &(0 as $t)
            }
        }
    )*)
}
impl_superior_zero! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }

impl Triangle {
    pub fn build<T: SuperiorZero>(sides: [T; 3]) -> Option<Triangle> {
        let mut s: Vec<T> = sides.to_vec();
        s.sort_unstable_by(|s1, s2| s1.partial_cmp(s2).unwrap_or(Greater));
        match (s[0], s[1], s[2]) {
            (a, _, _) if !a.is_strictly_superior_to_zero() => None,
            (a, b, c) if a + b < c => None,
            (a, b, c) if a == b && b == c => Some(Triangle(Equilateral)),
            (a, b, c) if a == b || b == c => Some(Triangle(Isosceles)),
            _ => Some(Triangle(Scalene)),
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.0 == Equilateral
    }

    pub fn is_scalene(&self) -> bool {
        self.0 == Scalene
    }

    pub fn is_isosceles(&self) -> bool {
        self.0 == Isosceles
    }
}
