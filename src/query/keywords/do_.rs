use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Do;

impl FromStr for Do {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DO" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword DO not found.".into(),
            ))),
        }
    }
}

impl Display for Do {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DO")
    }
}

impl SqliteKeyword for Do {}
