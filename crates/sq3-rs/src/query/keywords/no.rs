use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct No;

impl FromStr for No {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NO" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword NO not found.".into(),
            ))),
        }
    }
}

impl Display for No {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NO")
    }
}

impl SqliteKeyword for No {}
