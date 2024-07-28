use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Add;

impl FromStr for Add {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ADD" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword ADD not found.".into(),
            ))),
        }
    }
}

impl Display for Add {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ADD")
    }
}

impl SqliteKeyword for Add {}
