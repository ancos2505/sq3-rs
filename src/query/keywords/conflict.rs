use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Conflict;

impl FromStr for Conflict {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CONFLICT" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword CONFLICT not found.".into(),
            ))),
        }
    }
}

impl Display for Conflict {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CONFLICT")
    }
}

impl SqliteKeyword for Conflict {}
