use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Generated;

impl FromStr for Generated {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GENERATED" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword GENERATED not found.".into(),
            ))),
        }
    }
}

impl Display for Generated {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GENERATED")
    }
}

impl SqliteKeyword for Generated {}
