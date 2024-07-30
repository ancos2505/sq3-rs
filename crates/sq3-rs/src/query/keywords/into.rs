use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Into;

impl FromStr for Into {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "INTO" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword INTO not found.".into(),
            ))),
        }
    }
}

impl Display for Into {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "INTO")
    }
}

impl SqliteKeyword for Into {}
