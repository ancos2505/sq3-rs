use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Rename;

impl FromStr for Rename {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "RENAME" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword RENAME not found.".into(),
            ))),
        }
    }
}

impl Display for Rename {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RENAME")
    }
}

impl SqliteKeyword for Rename {}
