use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Full;

impl FromStr for Full {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "FULL" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword FULL not found.".into(),
            ))),
        }
    }
}

impl Display for Full {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FULL")
    }
}

impl SqliteKeyword for Full {}
