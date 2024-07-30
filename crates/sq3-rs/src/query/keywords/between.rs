use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Between;

impl FromStr for Between {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "BETWEEN" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword BETWEEN not found.".into(),
            ))),
        }
    }
}

impl Display for Between {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BETWEEN")
    }
}

impl SqliteKeyword for Between {}
