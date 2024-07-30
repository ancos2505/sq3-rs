use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct View;

impl FromStr for View {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "VIEW" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword VIEW not found.".into(),
            ))),
        }
    }
}

impl Display for View {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "VIEW")
    }
}

impl SqliteKeyword for View {}
