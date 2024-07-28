use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Restrict;

impl FromStr for Restrict {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "RESTRICT" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword RESTRICT not found.".into(),
            ))),
        }
    }
}

impl Display for Restrict {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RESTRICT")
    }
}

impl SqliteKeyword for Restrict {}
