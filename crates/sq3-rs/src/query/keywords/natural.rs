use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Natural;

impl FromStr for Natural {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NATURAL" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword NATURAL not found.".into(),
            ))),
        }
    }
}

impl Display for Natural {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NATURAL")
    }
}

impl SqliteKeyword for Natural {}
