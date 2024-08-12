use std::{
    error::Error,
    fmt::Display,
    num::{ParseFloatError, ParseIntError},
};

pub(crate) type ParserResult<T> = Result<T, Sq3ParserError>;

#[derive(Debug)]
pub struct Sq3ParserError(pub String);

impl From<ParseFloatError> for Sq3ParserError {
    fn from(error: ParseFloatError) -> Self {
        Self(error.to_string())
    }
}

impl From<ParseIntError> for Sq3ParserError {
    fn from(error: ParseIntError) -> Self {
        Self(error.to_string())
    }
}
impl Display for Sq3ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for Sq3ParserError {}
