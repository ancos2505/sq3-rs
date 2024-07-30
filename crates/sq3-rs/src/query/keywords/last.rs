use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Last;

impl FromStr for Last {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "LAST" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword LAST not found.".into(),
            ))),
        }
    }
}

impl Display for Last {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LAST")
    }
}

impl SqliteKeyword for Last {}
