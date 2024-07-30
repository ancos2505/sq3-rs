use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Limit;

impl FromStr for Limit {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "LIMIT" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword LIMIT not found.".into(),
            ))),
        }
    }
}

impl Display for Limit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LIMIT")
    }
}

impl SqliteKeyword for Limit {}
