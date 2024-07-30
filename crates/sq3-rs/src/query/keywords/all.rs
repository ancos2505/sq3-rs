use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::{DistinctProcessing, SqliteKeyword},
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct All;

impl FromStr for All {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ALL" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword ALL not found.".into(),
            ))),
        }
    }
}

impl Display for All {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ALL")
    }
}

impl SqliteKeyword for All {}
impl DistinctProcessing for All {}
