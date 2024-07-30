use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Cast;

impl FromStr for Cast {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CAST" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword CAST not found.".into(),
            ))),
        }
    }
}

impl Display for Cast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CAST")
    }
}

impl SqliteKeyword for Cast {}
