mod lang;
mod parser;

use std::ops::{Add, Div, Mul, Sub};
use crate::lang::{Operator, Zero};
use std::collections::HashMap;
use std::str::FromStr;
use nom::lib::std::fmt::Debug;

pub type Value = i32;
pub type ForthResult = Result<(), Error>;


#[derive(Default)]
pub struct Forth<T = Value> {
    stack: Vec<T>,
    words: HashMap<String, Vec<lang::Operator<T>>>,
}

impl<T> crate::lang::Stack<T> for Forth<T> {
    fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }
    fn push(&mut self, val: T) {
        self.stack.push(val)
    }
}

impl<T> FromStr for lang::Operator<T> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DROP" => Ok(lang::Operator::Drop),
            "DUP" => Ok(lang::Operator::Dup),
            "+" => Ok(lang::Operator::Add),
            "-" => Ok(lang::Operator::Sub),
            "*" => Ok(lang::Operator::Mul),
            "/" => Ok(lang::Operator::Div),
            "SWAP" => Ok(lang::Operator::Swap),
            "OVER" => Ok(lang::Operator::Over),
            _ => Err(()),
        }
    }
}


#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
    InvalidProgram,
}

impl From<lang::Error> for Error{
    fn from(e: lang::Error) -> Self {
        match e{
            lang::Error::DivisionByZero => Error::DivisionByZero,
            lang::Error::StackUnderflow => Error::StackUnderflow
        }
    }
}

impl From<parser::Error> for Error{
    fn from(e: parser::Error) -> Self {
        match e {
            parser::Error::InvalidWord => Error::InvalidWord,
            _ => Error::InvalidProgram
        }
    }
}

impl<T> Forth<T> {
    pub fn new() -> Forth<T> where T:Default {
        Forth::default()
    }

    pub fn stack(&self) -> &[T] {
        &self.stack
    }

    pub fn eval(&mut self, input: &str) -> ForthResult
    where T: FromStr + Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Zero,
        T::Err: Debug
    {
        for token in crate::parser::tokenize(input).map_err(Error::from)? {
            match token {
                parser::Token::Value(v) => {
                    lang::Operator::Push(v.parse().unwrap()).apply(self)?;
                }
                parser::Token::Word(w) => {
                    let w = w.to_ascii_uppercase();
                    if let Some(primitives) = self.words.get(&w) {
                        for primitive in primitives.to_owned() {
                            primitive.apply(self).map_err(Error::from)?
                        }
                    } else {
                        w.parse::<lang::Operator<T>>()
                            .map_err(|_| Error::UnknownWord)?
                            .apply(self)
                            .map_err(Error::from)?
                    }
                }
                parser::Token::WordDef((n, d)) => {
                    let primitives = self.eval_word_def(&d)?;
                    self.words.insert(n.to_ascii_uppercase(), primitives);
                }
            }
        }
        Ok(())
    }

    fn eval_word_def(&self, def: &[parser::Token]) -> Result<Vec<Operator<T>>, Error>
    where T: FromStr + Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Zero,
    T::Err: Debug
    {
        let mut primitives: Vec<Operator<T>> = Vec::with_capacity(def.len());
        for def in def {
            match def {
                parser::Token::Value(v) => {
                    primitives.push(Operator::Push(v.parse().unwrap()));
                }
                parser::Token::Word(w) => {
                    let w = w.to_ascii_uppercase();
                    if let Some(vec) = self.words.get(&w) {
                        primitives.extend(vec);
                    } else {
                        primitives.push(w.parse().map_err(|_| Error::InvalidWord)?);
                    }
                }
                parser::Token::WordDef(_) => {
                    return Err(Error::InvalidWord);
                }
            }
        }
        Ok(primitives)
    }
}
