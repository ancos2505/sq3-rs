use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct In;

impl FromStr for In {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "IN" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword IN not found.".into(),
            ))),
        }
    }
}

impl Display for In {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IN")
    }
}

impl SqliteKeyword for In {}
