use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Column;

impl FromStr for Column {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "COLUMN" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword COLUMN not found.".into(),
            ))),
        }
    }
}

impl Display for Column {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "COLUMN")
    }
}

impl SqliteKeyword for Column {}
