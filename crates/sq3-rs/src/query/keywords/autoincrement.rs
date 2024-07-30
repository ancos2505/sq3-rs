use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Autoincrement;

impl FromStr for Autoincrement {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AUTOINCREMENT" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword AUTOINCREMENT not found.".into(),
            ))),
        }
    }
}

impl Display for Autoincrement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AUTOINCREMENT")
    }
}

impl SqliteKeyword for Autoincrement {}
