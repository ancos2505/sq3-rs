use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Null;

impl FromStr for Null {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NULL" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword NULL not found.".into(),
            ))),
        }
    }
}

impl Display for Null {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NULL")
    }
}

impl SqliteKeyword for Null {}
