use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Insert;

impl FromStr for Insert {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "INSERT" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword INSERT not found.".into(),
            ))),
        }
    }
}

impl Display for Insert {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "INSERT")
    }
}

impl SqliteKeyword for Insert {}
