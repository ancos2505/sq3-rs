use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Of;

impl FromStr for Of {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "OF" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword OF not found.".into(),
            ))),
        }
    }
}

impl Display for Of {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OF")
    }
}

impl SqliteKeyword for Of {}
