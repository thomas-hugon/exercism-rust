pub fn brackets_are_balanced(string: &str) -> bool {
    let mut vec: Vec<char> = Vec::new();

    for c in string.chars() {
        match c {
            '(' => vec.push(')'),
            '{' => vec.push('}'),
            '[' => vec.push(']'),
            ')' | '}' | ']' if vec.pop() != Some(c) => return false,
            _ => {}
        }
    }
    vec.is_empty()
}
