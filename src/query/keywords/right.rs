use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Right;

impl FromStr for Right {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "RIGHT" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword RIGHT not found.".into(),
            ))),
        }
    }
}

impl Display for Right {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RIGHT")
    }
}

impl SqliteKeyword for Right {}
