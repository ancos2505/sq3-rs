use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Exclude;

impl FromStr for Exclude {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "EXCLUDE" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword EXCLUDE not found.".into(),
            ))),
        }
    }
}

impl Display for Exclude {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EXCLUDE")
    }
}

impl SqliteKeyword for Exclude {}
