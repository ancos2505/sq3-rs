use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Fail;

impl FromStr for Fail {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "FAIL" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword FAIL not found.".into(),
            ))),
        }
    }
}

impl Display for Fail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FAIL")
    }
}

impl SqliteKeyword for Fail {}
