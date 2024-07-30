use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Alter;

impl FromStr for Alter {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ALTER" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword ALTER not found.".into(),
            ))),
        }
    }
}

impl Display for Alter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ALTER")
    }
}

impl SqliteKeyword for Alter {}
