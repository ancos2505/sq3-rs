use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Partition;

impl FromStr for Partition {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PARTITION" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword PARTITION not found.".into(),
            ))),
        }
    }
}

impl Display for Partition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PARTITION")
    }
}

impl SqliteKeyword for Partition {}
