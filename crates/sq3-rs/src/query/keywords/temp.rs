use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Temp;

impl FromStr for Temp {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "TEMP" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword TEMP not found.".into(),
            ))),
        }
    }
}

impl Display for Temp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TEMP")
    }
}

impl SqliteKeyword for Temp {}
