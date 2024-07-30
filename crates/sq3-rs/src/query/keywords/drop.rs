use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Drop;

impl FromStr for Drop {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DROP" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword DROP not found.".into(),
            ))),
        }
    }
}

impl Display for Drop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DROP")
    }
}

impl SqliteKeyword for Drop {}
