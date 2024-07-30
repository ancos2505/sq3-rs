use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Constraint;

impl FromStr for Constraint {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CONSTRAINT" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword CONSTRAINT not found.".into(),
            ))),
        }
    }
}

impl Display for Constraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CONSTRAINT")
    }
}

impl SqliteKeyword for Constraint {}
