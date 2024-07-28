use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct First;

impl FromStr for First {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "FIRST" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword FIRST not found.".into(),
            ))),
        }
    }
}

impl Display for First {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FIRST")
    }
}

impl SqliteKeyword for First {}
