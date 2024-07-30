use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Nothing;

impl FromStr for Nothing {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NOTHING" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword NOTHING not found.".into(),
            ))),
        }
    }
}

impl Display for Nothing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NOTHING")
    }
}

impl SqliteKeyword for Nothing {}
