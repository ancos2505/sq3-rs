use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Key;

impl FromStr for Key {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "KEY" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword KEY not found.".into(),
            ))),
        }
    }
}

impl Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "KEY")
    }
}

impl SqliteKeyword for Key {}
