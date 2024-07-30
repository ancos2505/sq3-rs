use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Plan;

impl FromStr for Plan {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PLAN" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword PLAN not found.".into(),
            ))),
        }
    }
}

impl Display for Plan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PLAN")
    }
}

impl SqliteKeyword for Plan {}
