use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Outer;

impl FromStr for Outer {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "OUTER" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword OUTER not found.".into(),
            ))),
        }
    }
}

impl Display for Outer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OUTER")
    }
}

impl SqliteKeyword for Outer {}
