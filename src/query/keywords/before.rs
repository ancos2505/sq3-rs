use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Before;

impl FromStr for Before {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "BEFORE" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword BEFORE not found.".into(),
            ))),
        }
    }
}

impl Display for Before {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BEFORE")
    }
}

impl SqliteKeyword for Before {}
