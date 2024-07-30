use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Table;

impl FromStr for Table {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "TABLE" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword TABLE not found.".into(),
            ))),
        }
    }
}

impl Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TABLE")
    }
}

impl SqliteKeyword for Table {}
