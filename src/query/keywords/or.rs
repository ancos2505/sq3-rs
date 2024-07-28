use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Or;

impl FromStr for Or {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "OR" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword OR not found.".into(),
            ))),
        }
    }
}

impl Display for Or {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OR")
    }
}

impl SqliteKeyword for Or {}
