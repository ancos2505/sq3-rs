use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Action;

impl FromStr for Action {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ACTION" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword ACTION not found.".into(),
            ))),
        }
    }
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ACTION")
    }
}

impl SqliteKeyword for Action {}
