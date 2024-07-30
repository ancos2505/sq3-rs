use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct On;

impl FromStr for On {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ON" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword ON not found.".into(),
            ))),
        }
    }
}

impl Display for On {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ON")
    }
}

impl SqliteKeyword for On {}
