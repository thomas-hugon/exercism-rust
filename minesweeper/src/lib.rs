mod mines_board {
    pub struct Board<'a, 'b>(&'a [&'b str]);

    const DELTAS: [(i32, i32); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    impl<'a, 'b> Board<'a, 'b> {
        pub fn new(minefield: &'a [&'b str]) -> Board<'a, 'b> {
            Board(minefield)
        }

        pub fn annotate(&self) -> Vec<String> {
            self.0
                .iter()
                .enumerate()
                .map(|(row_index, row)| {
                    row.chars()
                        .enumerate()
                        .map(|(column_index, square)| match square {
                            '*' => square,
                            _ => self.count_mines(row_index, column_index),
                        })
                        .collect::<String>()
                })
                .collect::<Vec<String>>()
        }

        fn count_mines(&self, row: usize, column: usize) -> char {
            let surrounding_mines_count: u8 = DELTAS
                .iter()
                .map(|(dr, dc)| (dr + row as i32, dc + column as i32))
                .filter(|(row, column)| *row >= 0 && *column >= 0)
                .map(|(row, column)| match self.square_at(row, column) {
                    Some('*') => 1,
                    _ => 0,
                })
                .sum();

            match surrounding_mines_count {
                0 => ' ',
                _ => (surrounding_mines_count + b'0') as char,
            }
        }

        fn square_at(&self, row: i32, column: i32) -> Option<char> {
            self.0
                .get(row as usize)
                .and_then(|columns| columns.chars().nth(column as usize))
        }
    }
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    mines_board::Board::new(minefield).annotate()
}
