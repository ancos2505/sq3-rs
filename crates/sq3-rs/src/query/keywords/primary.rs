use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Primary;

impl FromStr for Primary {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PRIMARY" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword PRIMARY not found.".into(),
            ))),
        }
    }
}

impl Display for Primary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PRIMARY")
    }
}

impl SqliteKeyword for Primary {}
