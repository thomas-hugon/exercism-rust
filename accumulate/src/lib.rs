/// What should the type of _function be?
pub fn map<I, O, F>(input: Vec<I>, mut function: F) -> Vec<O>
    where F: FnMut(I) -> O {
    let mut output = Vec::with_capacity(input.len());
    for val in input {
        output.push(function(val));
    }

    output
}
