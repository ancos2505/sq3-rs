use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Collate;

impl FromStr for Collate {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "COLLATE" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword COLLATE not found.".into(),
            ))),
        }
    }
}

impl Display for Collate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "COLLATE")
    }
}

impl SqliteKeyword for Collate {}
