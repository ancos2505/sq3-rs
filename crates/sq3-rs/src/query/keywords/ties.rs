use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Ties;

impl FromStr for Ties {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "TIES" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword TIES not found.".into(),
            ))),
        }
    }
}

impl Display for Ties {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TIES")
    }
}

impl SqliteKeyword for Ties {}
