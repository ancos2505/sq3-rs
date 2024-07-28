use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Nulls;

impl FromStr for Nulls {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NULLS" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword NULLS not found.".into(),
            ))),
        }
    }
}

impl Display for Nulls {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NULLS")
    }
}

impl SqliteKeyword for Nulls {}
