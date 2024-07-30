use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Commit;

impl FromStr for Commit {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "COMMIT" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword COMMIT not found.".into(),
            ))),
        }
    }
}

impl Display for Commit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "COMMIT")
    }
}

impl SqliteKeyword for Commit {}
