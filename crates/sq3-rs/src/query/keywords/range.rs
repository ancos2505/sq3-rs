use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Range;

impl FromStr for Range {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "RANGE" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword RANGE not found.".into(),
            ))),
        }
    }
}

impl Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RANGE")
    }
}

impl SqliteKeyword for Range {}
