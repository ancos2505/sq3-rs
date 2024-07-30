use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Initially;

impl FromStr for Initially {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "INITIALLY" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword INITIALLY not found.".into(),
            ))),
        }
    }
}

impl Display for Initially {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "INITIALLY")
    }
}

impl SqliteKeyword for Initially {}
