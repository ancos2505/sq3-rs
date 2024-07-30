use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Intersect;

impl FromStr for Intersect {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "INTERSECT" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword INTERSECT not found.".into(),
            ))),
        }
    }
}

impl Display for Intersect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "INTERSECT")
    }
}

impl SqliteKeyword for Intersect {}
