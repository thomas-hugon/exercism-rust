use crate::Error::{InvalidDigit, SpanTooLong};

#[derive(Debug, PartialEq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    match (string_digits.len(), span) {
        (_, 0) => Ok(1),
        (strlen, span) if span > strlen => Err(SpanTooLong),
        _ => string_digits
            .chars()
            .collect::<Vec<char>>()
            .windows(span)
            .map(|w| {
                w.iter().try_fold(1, |acc, c| {
                    c.to_digit(10)
                        .ok_or_else(|| InvalidDigit(*c))
                        .map(|v| v as u64 * acc)
                })
            })
            .try_fold(0, |acc, p| p.map(|p| p.max(acc))),
    }
}
