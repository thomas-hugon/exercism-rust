use crate::Allergen::*;
use lazy_static::lazy_static;

pub struct Allergies(Vec<Allergen>);

#[derive(Debug, PartialEq, Clone)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

lazy_static! {
    static ref FACTORIES: Vec<fn() -> Allergen> = vec![
        || Eggs,
        || Peanuts,
        || Shellfish,
        || Strawberries,
        || Tomatoes,
        || Chocolate,
        || Pollen,
        || Cats,
    ];
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies(
            FACTORIES
                .iter()
                .enumerate()
                .filter(|(i, _)| 1 << i & score != 0)
                .map(|(_, f)| f())
                .collect(),
        )
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.0.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.0.to_vec()
    }
}
