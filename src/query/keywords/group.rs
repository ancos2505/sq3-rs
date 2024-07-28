use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Group;

impl FromStr for Group {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GROUP" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword GROUP not found.".into(),
            ))),
        }
    }
}

impl Display for Group {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GROUP")
    }
}

impl SqliteKeyword for Group {}
