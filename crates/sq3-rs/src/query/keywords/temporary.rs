use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Temporary;

impl FromStr for Temporary {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "TEMPORARY" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword TEMPORARY not found.".into(),
            ))),
        }
    }
}

impl Display for Temporary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TEMPORARY")
    }
}

impl SqliteKeyword for Temporary {}
