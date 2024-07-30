use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Like;

impl FromStr for Like {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "LIKE" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword LIKE not found.".into(),
            ))),
        }
    }
}

impl Display for Like {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LIKE")
    }
}

impl SqliteKeyword for Like {}
