pub struct CodonsInfo<'a> {
    codons: Vec<(&'a str, &'a str)>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        self.codons
            .iter()
            .filter(|(c, _)| &codon == c)
            .map(|(_, name)| *name)
            .next()
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        let mut vec = Vec::new();
        for i in 0..rna.len() / 3 {
            let name = self.name_for(&rna[i * 3..(i + 1) * 3]);
            if name? == "stop codon" {
                return Some(vec);
            }
            vec.push(name?);
        }
        Some(vec).filter(|vec| vec.is_empty() || rna.len() % 3 == 0)
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    CodonsInfo { codons: pairs }
}
