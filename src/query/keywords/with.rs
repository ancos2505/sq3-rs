use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct With;

impl FromStr for With {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "WITH" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword WITH not found.".into(),
            ))),
        }
    }
}

impl Display for With {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WITH")
    }
}

impl SqliteKeyword for With {}
