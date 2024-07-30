use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Window;

impl FromStr for Window {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "WINDOW" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword WINDOW not found.".into(),
            ))),
        }
    }
}

impl Display for Window {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WINDOW")
    }
}

impl SqliteKeyword for Window {}
