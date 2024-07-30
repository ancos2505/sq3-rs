use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Immediate;

impl FromStr for Immediate {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "IMMEDIATE" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword IMMEDIATE not found.".into(),
            ))),
        }
    }
}

impl Display for Immediate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IMMEDIATE")
    }
}

impl SqliteKeyword for Immediate {}
