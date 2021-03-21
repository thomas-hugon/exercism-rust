#[derive(Debug, PartialEq)]
pub struct Dna(String);

#[derive(Debug, PartialEq)]
pub struct Rna(String);

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        dna.find(|c: char| !"GCTA".contains(c))
            .map_or_else(|| Ok(Dna(String::from(dna))), Err)
    }

    pub fn into_rna(self) -> Rna {
        Rna(self
            .0
            .chars()
            .map(|c| match c {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                _ => 'U',
            })
            .collect())
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        rna.find(|c: char| !"GCUA".contains(c))
            .map_or_else(|| Ok(Rna(String::from(rna))), Err)
    }
}
