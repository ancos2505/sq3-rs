use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Virtual;

impl FromStr for Virtual {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "VIRTUAL" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword VIRTUAL not found.".into(),
            ))),
        }
    }
}

impl Display for Virtual {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "VIRTUAL")
    }
}

impl SqliteKeyword for Virtual {}
