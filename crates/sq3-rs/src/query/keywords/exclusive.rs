use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Exclusive;

impl FromStr for Exclusive {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "EXCLUSIVE" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword EXCLUSIVE not found.".into(),
            ))),
        }
    }
}

impl Display for Exclusive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EXCLUSIVE")
    }
}

impl SqliteKeyword for Exclusive {}
