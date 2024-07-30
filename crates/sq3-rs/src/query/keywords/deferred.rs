use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Deferred;

impl FromStr for Deferred {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DEFERRED" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword DEFERRED not found.".into(),
            ))),
        }
    }
}

impl Display for Deferred {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DEFERRED")
    }
}

impl SqliteKeyword for Deferred {}
