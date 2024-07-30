use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Attach;

impl FromStr for Attach {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ATTACH" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword ATTACH not found.".into(),
            ))),
        }
    }
}

impl Display for Attach {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ATTACH")
    }
}

impl SqliteKeyword for Attach {}
