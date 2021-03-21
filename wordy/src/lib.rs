use lazy_static::lazy_static;
use regex::*;

type Operator = fn(i32, i32) -> i32;

struct ParsedOperation {
    operator: Operator,
    second_operand: i32,
    remaining: &'static str,
}

lazy_static! {
    static ref OPERATIONS: [(Regex, Operator); 6] = [
        (Regex::new(r"^\s*plus\s+(-?\d+)(.*)$").unwrap(), |a, b| a+b),
        (Regex::new(r"^\s*minus\s+(-?\d+)(.*)$").unwrap(), |a, b| a-b),
        (Regex::new(r"^\s*divided\s+by\s+(-?\d+)(.*)$").unwrap(), |a, b| a/b),
        (Regex::new(r"^\s*multiplied\s+by\s+(-?\d+)(.*)$").unwrap(), |a, b| a*b),
        (Regex::new(r"^\s*raised\s+to\s+the\s+(\d+)(?:st|nd|rd|th)\s+power(.*)$").unwrap(), |a, b| a.pow(b as u32)),
        (Regex::new(r"^What\s+is\s+(-?\d+)(.*\?)$").unwrap(), |_, b| b),
    ];
}

impl ParsedOperation {
    pub fn parse(text: &'static str) -> Option<ParsedOperation> {
        for (regex, op) in OPERATIONS.iter() {
            if let Some(captures) = regex.captures(text) {
                return captures.get(1).and_then(|m_nb| {
                    captures.get(2).map(|m_rem| ParsedOperation {
                        operator: *op,
                        second_operand: m_nb.as_str().parse::<i32>().unwrap(),
                        remaining: m_rem.as_str(),
                    })
                });
            }
        }
        None
    }

    pub fn apply(&self, first_operand: i32) -> i32 {
        (self.operator)(first_operand, self.second_operand)
    }
}

pub fn answer(mut command: &'static str) -> Option<i32> {
    let mut result = 0;
    while let Some(operation) = ParsedOperation::parse(command) {
        result = operation.apply(result);
        command = operation.remaining;
    }

    Some(result).filter(|_| command == "?")
}
