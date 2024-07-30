use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Materialized;

impl FromStr for Materialized {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "MATERIALIZED" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword MATERIALIZED not found.".into(),
            ))),
        }
    }
}

impl Display for Materialized {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MATERIALIZED")
    }
}

impl SqliteKeyword for Materialized {}
