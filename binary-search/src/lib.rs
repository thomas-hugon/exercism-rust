pub fn find<T: Copy + Ord, X: AsRef<[T]>>(array: X, key: T) -> Option<usize> {
    fn find<T: Copy + Ord, X: AsRef<[T]>>(array: X, key: T, index: usize) -> Option<usize> {
        let array = array.as_ref();
        match (array.len() / 2, array[array.len() / 2]) {
            (0, a) if a != key => None,
            (i, a) if key < a => find(&array[0..i], key, index),
            (i, a) if key > a => find(&array[i..array.len()], key, index + i),
            (i, _) => Some(index + i),
        }
    }
    let array = array.as_ref();
    match array.len() {
        0 => None,
        _ => find(array, key, 0),
    }
}
