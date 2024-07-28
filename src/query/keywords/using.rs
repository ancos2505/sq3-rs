use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Using;

impl FromStr for Using {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "USING" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword USING not found.".into(),
            ))),
        }
    }
}

impl Display for Using {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "USING")
    }
}

impl SqliteKeyword for Using {}
