use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Trigger;

impl FromStr for Trigger {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "TRIGGER" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword TRIGGER not found.".into(),
            ))),
        }
    }
}

impl Display for Trigger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TRIGGER")
    }
}

impl SqliteKeyword for Trigger {}
