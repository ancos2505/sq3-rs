use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Rollback;

impl FromStr for Rollback {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ROLLBACK" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword ROLLBACK not found.".into(),
            ))),
        }
    }
}

impl Display for Rollback {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ROLLBACK")
    }
}

impl SqliteKeyword for Rollback {}
