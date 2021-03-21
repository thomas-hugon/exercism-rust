use std::fmt::Debug;

#[derive(Debug)]
pub struct CustomSet<T> {
    data: Vec<T>,
}

impl<T: PartialEq + Clone> PartialEq for CustomSet<T> {
    fn eq(&self, other: &Self) -> bool {
        self.data.len() == other.data.len() && self.data.iter().all(|elem| other.contains(elem))
    }
}

impl<T: PartialEq + Clone> CustomSet<T> {
    pub fn new(input: &[T]) -> Self {
        CustomSet {
            data: input.to_vec(),
        }
    }

    pub fn contains(&self, element: &T) -> bool {
        self.data.iter().any(|e| e == element)
    }

    pub fn add(&mut self, element: T) {
        if !self.contains(&element) {
            self.data.push(element);
        }
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        &self.intersection(other) == self
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.intersection(other).is_empty()
    }

    pub fn intersection(&self, other: &Self) -> Self {
        CustomSet {
            data: self
                .data
                .iter()
                .filter(|elem| other.contains(elem))
                .cloned()
                .collect(),
        }
    }

    pub fn difference(&self, other: &Self) -> Self {
        CustomSet {
            data: self
                .data
                .iter()
                .filter(|elem| !other.contains(elem))
                .cloned()
                .collect(),
        }
    }

    pub fn union(&self, other: &Self) -> Self {
        CustomSet {
            data: self
                .data
                .iter()
                .filter(|elem| !other.contains(elem))
                .cloned()
                .chain(other.data.iter().cloned())
                .collect(),
        }
    }
}
