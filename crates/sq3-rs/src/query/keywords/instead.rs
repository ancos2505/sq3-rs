use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Instead;

impl FromStr for Instead {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "INSTEAD" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword INSTEAD not found.".into(),
            ))),
        }
    }
}

impl Display for Instead {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "INSTEAD")
    }
}

impl SqliteKeyword for Instead {}
