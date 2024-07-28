use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Returning;

impl FromStr for Returning {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "RETURNING" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword RETURNING not found.".into(),
            ))),
        }
    }
}

impl Display for Returning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RETURNING")
    }
}

impl SqliteKeyword for Returning {}
