use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Inner;

impl FromStr for Inner {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "INNER" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword INNER not found.".into(),
            ))),
        }
    }
}

impl Display for Inner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "INNER")
    }
}

impl SqliteKeyword for Inner {}
