use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Savepoint;

impl FromStr for Savepoint {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SAVEPOINT" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword SAVEPOINT not found.".into(),
            ))),
        }
    }
}

impl Display for Savepoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SAVEPOINT")
    }
}

impl SqliteKeyword for Savepoint {}
