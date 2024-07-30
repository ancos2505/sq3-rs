use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct And;

impl FromStr for And {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AND" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword AND not found.".into(),
            ))),
        }
    }
}

impl Display for And {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AND")
    }
}

impl SqliteKeyword for And {}
