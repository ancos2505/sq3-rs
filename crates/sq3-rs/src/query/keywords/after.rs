use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct After;

impl FromStr for After {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AFTER" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword AFTER not found.".into(),
            ))),
        }
    }
}

impl Display for After {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AFTER")
    }
}

impl SqliteKeyword for After {}
