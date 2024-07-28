use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Deferrable;

impl FromStr for Deferrable {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DEFERRABLE" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword DEFERRABLE not found.".into(),
            ))),
        }
    }
}

impl Display for Deferrable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DEFERRABLE")
    }
}

impl SqliteKeyword for Deferrable {}
