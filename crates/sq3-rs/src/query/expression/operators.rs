use std::str::FromStr;

use crate::traits::TypeName;
use sq3_derive::Name;

use crate::result::{SqlParserError, SqliteError};

#[derive(Debug, Name)]
pub struct Tilde;

impl FromStr for Tilde {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "~" => Ok(Self),

            _ => Err(SqliteError::SqlParser(SqlParserError(format!(
                "Error on parsing {}",
                Self::NAME,
            )))),
        }
    }
}

#[derive(Debug)]
pub struct Plus;

impl FromStr for Plus {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(format!(
                "Error on parsing {}",
                stringify!(Self)
            )))),
        }
    }
}

#[derive(Debug)]
pub struct Minus;

impl FromStr for Minus {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "-" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(format!(
                "Error on parsing tilde"
            )))),
        }
    }
}

#[derive(Debug)]
pub struct Multiplication;

#[derive(Debug)]
pub struct Percent;

#[derive(Debug)]
pub struct LeftParenthesis;

#[derive(Debug)]
pub struct RightParenthesis;

#[cfg(test)]
mod tests {
    use super::Tilde;

    #[test]
    fn ok_on_parse_tilde() {
        let res = "~".parse::<Tilde>();
        dbg!(&res);
        assert!(res.is_ok());
    }

    #[test]
    fn err_on_parse_tilde() {
        let res = "".parse::<Tilde>();
        dbg!(&res);
        assert!(res.is_err());
    }
}
