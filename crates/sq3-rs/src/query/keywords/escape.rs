use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Escape;

impl FromStr for Escape {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ESCAPE" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword ESCAPE not found.".into(),
            ))),
        }
    }
}

impl Display for Escape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ESCAPE")
    }
}

impl SqliteKeyword for Escape {}
