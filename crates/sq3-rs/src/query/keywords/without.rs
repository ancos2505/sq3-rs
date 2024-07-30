use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Without;

impl FromStr for Without {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "WITHOUT" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword WITHOUT not found.".into(),
            ))),
        }
    }
}

impl Display for Without {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WITHOUT")
    }
}

impl SqliteKeyword for Without {}
