use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Replace;

impl FromStr for Replace {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "REPLACE" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword REPLACE not found.".into(),
            ))),
        }
    }
}

impl Display for Replace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "REPLACE")
    }
}

impl SqliteKeyword for Replace {}
