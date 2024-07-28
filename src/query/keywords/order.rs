use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Order;

impl FromStr for Order {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ORDER" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword ORDER not found.".into(),
            ))),
        }
    }
}

impl Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ORDER")
    }
}

impl SqliteKeyword for Order {}
