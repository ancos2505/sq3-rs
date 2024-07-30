use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Cascade;

impl FromStr for Cascade {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CASCADE" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword CASCADE not found.".into(),
            ))),
        }
    }
}

impl Display for Cascade {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CASCADE")
    }
}

impl SqliteKeyword for Cascade {}
