use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Current_date;

impl FromStr for Current_date {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CURRENT_DATE" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword CURRENT_DATE not found.".into(),
            ))),
        }
    }
}

impl Display for Current_date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CURRENT_DATE")
    }
}

impl SqliteKeyword for Current_date {}
