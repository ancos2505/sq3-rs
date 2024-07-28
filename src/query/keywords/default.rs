use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Default;

impl FromStr for Default {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DEFAULT" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword DEFAULT not found.".into(),
            ))),
        }
    }
}

impl Display for Default {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DEFAULT")
    }
}

impl SqliteKeyword for Default {}
