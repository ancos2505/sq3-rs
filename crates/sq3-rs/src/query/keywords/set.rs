use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Set;

impl FromStr for Set {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SET" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword SET not found.".into(),
            ))),
        }
    }
}

impl Display for Set {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SET")
    }
}

impl SqliteKeyword for Set {}
