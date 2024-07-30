use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Abort;

impl FromStr for Abort {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ABORT" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword ABORT not found.".into(),
            ))),
        }
    }
}

impl Display for Abort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ABORT")
    }
}

impl SqliteKeyword for Abort {}
