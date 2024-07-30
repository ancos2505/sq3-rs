use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Asc;

impl FromStr for Asc {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ASC" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword ASC not found.".into(),
            ))),
        }
    }
}

impl Display for Asc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ASC")
    }
}

impl SqliteKeyword for Asc {}
