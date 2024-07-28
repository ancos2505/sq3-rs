use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Ignore;

impl FromStr for Ignore {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "IGNORE" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword IGNORE not found.".into(),
            ))),
        }
    }
}

impl Display for Ignore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IGNORE")
    }
}

impl SqliteKeyword for Ignore {}
