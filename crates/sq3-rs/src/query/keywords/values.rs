use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Values;

impl FromStr for Values {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "VALUES" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword VALUES not found.".into(),
            ))),
        }
    }
}

impl Display for Values {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "VALUES")
    }
}

impl SqliteKeyword for Values {}
