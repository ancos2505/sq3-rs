use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct As;

impl FromStr for As {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AS" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword AS not found.".into(),
            ))),
        }
    }
}

impl Display for As {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AS")
    }
}

impl SqliteKeyword for As {}
