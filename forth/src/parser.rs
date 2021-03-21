use nom::IResult;
use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::character::complete::{char, digit1, multispace0};

use nom::combinator::{map, recognize};
use nom::error::{ParseError, ErrorKind};
use nom::multi::{many0, many1};
use nom::sequence::{delimited, pair, preceded, terminated};

#[derive(Debug)]
pub enum Token<'a>{
    Value(&'a str),
    Word(&'a str),
    WordDef((&'a str, Vec<Token<'a>>)),
}

#[derive(Debug)]
enum ParsingError<'a> {
    InvalidWord,
    ParsingError(nom::Err<nom::error::Error<&'a str>>)
}

impl<'a> ParseError<&'a str> for ParsingError<'a>{
    fn from_error_kind(input: &'a str, code: ErrorKind) -> Self {
        Self::ParsingError(nom::Err::Error(nom::error::Error{
            input,
            code
        }))
    }

    fn append(_input: &str, _kind: ErrorKind, other: Self) -> Self {
        other
    }
}

fn wrapped_spaces<'a, F: 'a, O, E: ParseError<&'a str>>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
    where
        F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    delimited( multispace0, inner, multispace0 )
}


fn value_str(input: &str) -> IResult<&str, &str, ParsingError> {
    wrapped_spaces(digit1 )(input)
}

fn token_value(input: &str) -> IResult<&str, Token, ParsingError> {
    map(value_str, Token::Value)(input)
}

fn word_str(input: &str) -> IResult<&str, &str, ParsingError> {
    wrapped_spaces(recognize(
        pair(
            is_not("0123456789 ;:\t\r\n"),
            many0(is_not(" ;:\t\r\n"))
        )
    ) )(input)
}

fn token_word(input: &str) -> IResult<&str, Token, ParsingError> {
    map(word_str, Token::Word)(input)
}

fn word_def_inner(input: &str) -> IResult<&str, (&str, Vec<Token>), ParsingError> {
    terminated(
        pair(word_str, many1(alt((token_word, token_value))))
        ,char(';')
    )(input).map_err(|_|nom::Err::Failure(ParsingError::InvalidWord))
}
fn word_def(input: &str) -> IResult<&str, (&str, Vec<Token>), ParsingError> {
    wrapped_spaces(preceded(char(':'), word_def_inner) )(input)
}

fn token_word_def(input: &str) -> IResult<&str, Token, ParsingError> {
    map(word_def, Token::WordDef)(input)
}

pub enum Error {
    InvalidWord,
    ParsingError
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, Error> {
    let result = many0(alt((token_word_def, token_word, token_value)))(input);
    match result {
        Ok((rest, tokens)) if rest.is_empty() => Ok(tokens),
        Err(nom::Err::Failure(ParsingError::InvalidWord)) => Err(Error::InvalidWord),
        _ => Err(Error::ParsingError),
    }
}



