use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Always;

impl FromStr for Always {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ALWAYS" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword ALWAYS not found.".into(),
            ))),
        }
    }
}

impl Display for Always {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ALWAYS")
    }
}

impl SqliteKeyword for Always {}
