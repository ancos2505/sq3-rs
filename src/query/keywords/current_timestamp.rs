use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Current_timestamp;

impl FromStr for Current_timestamp {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CURRENT_TIMESTAMP" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword CURRENT_TIMESTAMP not found.".into(),
            ))),
        }
    }
}

impl Display for Current_timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CURRENT_TIMESTAMP")
    }
}

impl SqliteKeyword for Current_timestamp {}
