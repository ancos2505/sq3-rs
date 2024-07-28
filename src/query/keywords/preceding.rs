use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Preceding;

impl FromStr for Preceding {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PRECEDING" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword PRECEDING not found.".into(),
            ))),
        }
    }
}

impl Display for Preceding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PRECEDING")
    }
}

impl SqliteKeyword for Preceding {}
