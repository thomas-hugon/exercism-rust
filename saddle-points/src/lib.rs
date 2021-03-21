pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    input
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, val)| (i, row, j, val)))
        .filter(|(_, row, _, val)| val == &row.iter().max().unwrap())
        .filter(|(_, _, j, val)| !input.iter().map(|row| row[*j]).any(|x| &x < val))
        .map(|(i, _, j, _)| (i, j))
        .collect()
}
