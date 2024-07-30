use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Query;

impl FromStr for Query {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "QUERY" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword QUERY not found.".into(),
            ))),
        }
    }
}

impl Display for Query {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "QUERY")
    }
}

impl SqliteKeyword for Query {}
