// You should change this.
//
// Depending on your implementation, there are a variety of potential errors
// which might occur. They aren't checked by the test suite in order to
// allow the greatest freedom of implementation, but real libraries should
// provide useful, descriptive errors so that downstream code can react
// appropriately.
//
// One common idiom is to define an Error enum which wraps all potential
// errors. Another common idiom is to use a helper type such as failure::Error
// which does more or less the same thing but automatically.
#[derive(Debug)]
pub enum Error {
    NotATonic,
    NotAInterval,
}

pub struct Scale {
    intervals: Vec<bool>,
    scale: Vec<&'static str>
}

impl Scale {
    const SHARPED: [&'static str; 12] = ["A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#"];
    const FLATTED: [&'static str; 12] = ["A", "Bb", "B", "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab"];

    pub fn new(tonic: &str, intervals: &str) -> Result<Scale, Error> {
        let scale = match tonic {
            "C" | "a" | "G" | "D" | "A" | "E" | "B" | "F#" | "e" | "b" | "f#" | "c#" | "g#" | "d#" => {
                Ok(&Self::SHARPED[..])
            }
            "F" | "Bb" | "Eb" | "Ab" | "Db" | "Gb" | "d" | "g" | "c" | "f" | "bb" | "eb" => {
                Ok(&Self::FLATTED[..])
            }
            _ => Err(Error::NotATonic),
        }?;

        let tonic = Self::uppercase_first_letter(tonic);

        let intervals = intervals.chars().try_fold(vec![true], |mut acc: Vec<bool>, b| {
            acc.extend_from_slice(match b {
                'm' => Ok(&[true][..]),
                'M' => Ok(&[false, true][..]),
                'A' => Ok(&[false, false, true][..]),
                _ => Err(Error::NotAInterval)
            }?);
            Ok(acc)
        })?;

        let tonic = scale.iter().position(|&s| s == tonic).unwrap();
        let scale = Self::rotate(scale, tonic);

        Ok(Scale { intervals, scale })
    }

    fn uppercase_first_letter(text: &str) -> String {
        text.chars()
            .take(1)
            .map(|s| s.to_ascii_uppercase())
            .chain(text.chars().skip(1))
            .collect()
    }

    fn rotate<T: Copy>(slice: &[T], shift_by: usize) -> Vec<T> {
        slice.iter()
            .cycle()
            .skip(shift_by)
            .take(slice.len())
            .copied().collect()
    }

    pub fn chromatic(tonic: &str) -> Result<Scale, Error> {
        Self::new(tonic, "mmmmmmmmmmmm")
    }

    pub fn enumerate(&self) -> Vec<String> {
        self.scale.iter().zip(self.intervals.iter())
            .filter_map(|(note, take)| take.then(|| note.to_string()))
            .collect()
    }
}
