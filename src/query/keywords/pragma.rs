use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Pragma;

impl FromStr for Pragma {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PRAGMA" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword PRAGMA not found.".into(),
            ))),
        }
    }
}

impl Display for Pragma {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PRAGMA")
    }
}

impl SqliteKeyword for Pragma {}
