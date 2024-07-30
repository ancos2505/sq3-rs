use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Index;

impl FromStr for Index {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "INDEX" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword INDEX not found.".into(),
            ))),
        }
    }
}

impl Display for Index {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "INDEX")
    }
}

impl SqliteKeyword for Index {}
