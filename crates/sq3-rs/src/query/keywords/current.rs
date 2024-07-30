use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Current;

impl FromStr for Current {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CURRENT" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword CURRENT not found.".into(),
            ))),
        }
    }
}

impl Display for Current {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CURRENT")
    }
}

impl SqliteKeyword for Current {}
