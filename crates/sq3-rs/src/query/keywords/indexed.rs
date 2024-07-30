use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Indexed;

impl FromStr for Indexed {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "INDEXED" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword INDEXED not found.".into(),
            ))),
        }
    }
}

impl Display for Indexed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "INDEXED")
    }
}

impl SqliteKeyword for Indexed {}
