use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Except;

impl FromStr for Except {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "EXCEPT" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword EXCEPT not found.".into(),
            ))),
        }
    }
}

impl Display for Except {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EXCEPT")
    }
}

impl SqliteKeyword for Except {}
