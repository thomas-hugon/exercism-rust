mod pascal {
    pub struct PascalsTriangle(u32);

    impl PascalsTriangle {
        pub fn new(row_count: u32) -> Self {
            PascalsTriangle(row_count)
        }

        pub fn rows(&self) -> Vec<Vec<u32>> {
            let mut vec: Vec<Vec<u32>> = Vec::new();
            while vec.len() < self.0 as usize {
                vec.push(next_row(vec.last()))
            }
            vec
        }
    }

    fn next_row(last_row: Option<&Vec<u32>>) -> Vec<u32> {
        let mut row = Vec::new();
        row.push(1);
        if let Some(last) = last_row {
            last.windows(2)
                .map(|w| w[0] + w[1])
                .for_each(|val| row.push(val));
            row.push(1);
        }
        row
    }
}

pub use pascal::PascalsTriangle;
