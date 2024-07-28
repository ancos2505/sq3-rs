use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Row;

impl FromStr for Row {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ROW" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword ROW not found.".into(),
            ))),
        }
    }
}

impl Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ROW")
    }
}

impl SqliteKeyword for Row {}
