use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct When;

impl FromStr for When {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "WHEN" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword WHEN not found.".into(),
            ))),
        }
    }
}

impl Display for When {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WHEN")
    }
}

impl SqliteKeyword for When {}
