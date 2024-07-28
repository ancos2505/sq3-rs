use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Match;

impl FromStr for Match {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "MATCH" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword MATCH not found.".into(),
            ))),
        }
    }
}

impl Display for Match {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MATCH")
    }
}

impl SqliteKeyword for Match {}
