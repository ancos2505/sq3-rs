use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Detach;

impl FromStr for Detach {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DETACH" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword DETACH not found.".into(),
            ))),
        }
    }
}

impl Display for Detach {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DETACH")
    }
}

impl SqliteKeyword for Detach {}
