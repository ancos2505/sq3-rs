use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Unbounded;

impl FromStr for Unbounded {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "UNBOUNDED" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword UNBOUNDED not found.".into(),
            ))),
        }
    }
}

impl Display for Unbounded {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UNBOUNDED")
    }
}

impl SqliteKeyword for Unbounded {}
