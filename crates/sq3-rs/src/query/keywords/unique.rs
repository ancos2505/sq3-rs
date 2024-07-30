use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Unique;

impl FromStr for Unique {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "UNIQUE" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword UNIQUE not found.".into(),
            ))),
        }
    }
}

impl Display for Unique {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UNIQUE")
    }
}

impl SqliteKeyword for Unique {}
