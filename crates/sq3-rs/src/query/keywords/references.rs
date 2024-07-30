use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct References;

impl FromStr for References {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "REFERENCES" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword REFERENCES not found.".into(),
            ))),
        }
    }
}

impl Display for References {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "REFERENCES")
    }
}

impl SqliteKeyword for References {}
