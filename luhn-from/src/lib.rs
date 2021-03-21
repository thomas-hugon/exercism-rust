pub struct Luhn(String);

impl Luhn {
    pub fn is_valid(&self) -> bool {
        luhn::is_valid(&self.0)
    }
}

impl<X: ToString> From<X> for Luhn {
    fn from(input: X) -> Self {
        Luhn(input.to_string())
    }
}
