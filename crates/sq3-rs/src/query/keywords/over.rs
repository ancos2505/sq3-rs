use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Over;

impl FromStr for Over {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "OVER" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword OVER not found.".into(),
            ))),
        }
    }
}

impl Display for Over {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OVER")
    }
}

impl SqliteKeyword for Over {}
