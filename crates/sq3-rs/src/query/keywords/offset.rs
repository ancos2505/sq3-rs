use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Offset;

impl FromStr for Offset {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "OFFSET" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword OFFSET not found.".into(),
            ))),
        }
    }
}

impl Display for Offset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OFFSET")
    }
}

impl SqliteKeyword for Offset {}
