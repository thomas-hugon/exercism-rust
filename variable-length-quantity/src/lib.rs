use crate::Error::{IncompleteNumber, Overflow};

#[derive(Debug, PartialEq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut vec = Vec::new();
    for value in values {
        let shifted_value = value >> 28;
        if shifted_value > 0 {
            vec.push(((127 & shifted_value) | 128) as u8)
        }
        let shifted_value = value >> 21;
        if shifted_value > 0 {
            vec.push(((127 & shifted_value) | 128) as u8)
        }
        let shifted_value = value >> 14;
        if shifted_value > 0 {
            vec.push(((127 & shifted_value) | 128) as u8)
        }
        let shifted_value = value >> 7;
        if shifted_value > 0 {
            vec.push(((127 & shifted_value) | 128) as u8)
        }
        vec.push((value & 127) as u8)
    }
    vec
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut vec = Vec::new();
    let mut curr: u32 = 0;
    for byte in bytes {
        curr = curr
            .checked_mul(128)
            .and_then(|not_overflowed| not_overflowed.checked_add((byte & 127) as u32))
            .ok_or(Overflow)?;
        if 128 & byte == 0 {
            vec.push(curr);
            curr = 0;
        }
    }
    if curr != 0 || bytes.last().map_or(false, |last| last & 128 == 128) {
        Err(IncompleteNumber)
    } else {
        Ok(vec)
    }
}
