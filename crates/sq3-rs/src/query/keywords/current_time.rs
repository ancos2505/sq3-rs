use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Current_time;

impl FromStr for Current_time {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CURRENT_TIME" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword CURRENT_TIME not found.".into(),
            ))),
        }
    }
}

impl Display for Current_time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CURRENT_TIME")
    }
}

impl SqliteKeyword for Current_time {}
