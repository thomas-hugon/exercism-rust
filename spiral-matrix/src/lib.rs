use std::ops::RangeFrom;

pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    fn update(mut mat: Vec<Vec<u32>>, start: usize, mut n: RangeFrom<u32>) -> Vec<Vec<u32>> {
        let end = mat.len() - start;
        if end < start + 1 {
            return mat;
        }
        for i in start..end {
            mat[start][i] = n.next().unwrap();
        }
        for i in (start + 1)..end {
            mat[i][end - 1] = n.next().unwrap();
        }
        for i in (start..(end - 1)).rev() {
            mat[end - 1][i] = n.next().unwrap();
        }
        for i in ((start + 1)..(end - 1)).rev() {
            mat[i][start] = n.next().unwrap();
        }

        update(mat, start + 1, n)
    }

    update(vec![vec![0u32; size as usize]; size as usize], 0, 1..)
}
