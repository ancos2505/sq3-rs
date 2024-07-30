use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct For;

impl FromStr for For {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "FOR" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword FOR not found.".into(),
            ))),
        }
    }
}

impl Display for For {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FOR")
    }
}

impl SqliteKeyword for For {}
