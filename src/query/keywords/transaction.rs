use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Transaction;

impl FromStr for Transaction {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "TRANSACTION" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword TRANSACTION not found.".into(),
            ))),
        }
    }
}

impl Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TRANSACTION")
    }
}

impl SqliteKeyword for Transaction {}
