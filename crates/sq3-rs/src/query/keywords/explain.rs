use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Explain;

impl FromStr for Explain {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "EXPLAIN" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword EXPLAIN not found.".into(),
            ))),
        }
    }
}

impl Display for Explain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EXPLAIN")
    }
}

impl SqliteKeyword for Explain {}
