use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Case;

impl FromStr for Case {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CASE" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword CASE not found.".into(),
            ))),
        }
    }
}

impl Display for Case {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CASE")
    }
}

impl SqliteKeyword for Case {}
