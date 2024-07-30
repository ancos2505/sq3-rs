use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Else;

impl FromStr for Else {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ELSE" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword ELSE not found.".into(),
            ))),
        }
    }
}

impl Display for Else {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ELSE")
    }
}

impl SqliteKeyword for Else {}
