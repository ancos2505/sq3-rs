use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Each;

impl FromStr for Each {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "EACH" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword EACH not found.".into(),
            ))),
        }
    }
}

impl Display for Each {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EACH")
    }
}

impl SqliteKeyword for Each {}
