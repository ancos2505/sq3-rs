use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Regexp;

impl FromStr for Regexp {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "REGEXP" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword REGEXP not found.".into(),
            ))),
        }
    }
}

impl Display for Regexp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "REGEXP")
    }
}

impl SqliteKeyword for Regexp {}
