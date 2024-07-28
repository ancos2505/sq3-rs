use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Notnull;

impl FromStr for Notnull {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NOTNULL" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword NOTNULL not found.".into(),
            ))),
        }
    }
}

impl Display for Notnull {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NOTNULL")
    }
}

impl SqliteKeyword for Notnull {}
