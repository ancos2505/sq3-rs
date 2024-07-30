use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Left;

impl FromStr for Left {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "LEFT" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword LEFT not found.".into(),
            ))),
        }
    }
}

impl Display for Left {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LEFT")
    }
}

impl SqliteKeyword for Left {}
