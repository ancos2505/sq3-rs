use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Then;

impl FromStr for Then {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "THEN" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword THEN not found.".into(),
            ))),
        }
    }
}

impl Display for Then {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "THEN")
    }
}

impl SqliteKeyword for Then {}
