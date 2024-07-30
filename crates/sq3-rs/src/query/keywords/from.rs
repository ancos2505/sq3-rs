use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct KeywordFrom;

impl FromStr for KeywordFrom {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "FROM" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword FROM not found.".into(),
            ))),
        }
    }
}

impl Display for KeywordFrom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FROM")
    }
}

impl SqliteKeyword for KeywordFrom {}
