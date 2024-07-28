use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Raise;

impl FromStr for Raise {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "RAISE" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword RAISE not found.".into(),
            ))),
        }
    }
}

impl Display for Raise {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RAISE")
    }
}

impl SqliteKeyword for Raise {}
