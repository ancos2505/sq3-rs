use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Glob;

impl FromStr for Glob {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GLOB" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword GLOB not found.".into(),
            ))),
        }
    }
}

impl Display for Glob {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GLOB")
    }
}

impl SqliteKeyword for Glob {}
