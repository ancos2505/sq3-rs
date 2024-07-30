use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Rows;

impl FromStr for Rows {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ROWS" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword ROWS not found.".into(),
            ))),
        }
    }
}

impl Display for Rows {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ROWS")
    }
}

impl SqliteKeyword for Rows {}
