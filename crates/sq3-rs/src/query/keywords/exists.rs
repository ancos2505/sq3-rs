use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Exists;

impl FromStr for Exists {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "EXISTS" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword EXISTS not found.".into(),
            ))),
        }
    }
}

impl Display for Exists {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EXISTS")
    }
}

impl SqliteKeyword for Exists {}
