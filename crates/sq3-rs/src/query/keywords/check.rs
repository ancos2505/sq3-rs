use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Check;

impl FromStr for Check {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CHECK" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword CHECK not found.".into(),
            ))),
        }
    }
}

impl Display for Check {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CHECK")
    }
}

impl SqliteKeyword for Check {}
