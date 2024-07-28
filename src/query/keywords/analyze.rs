use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Analyze;

impl FromStr for Analyze {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ANALYZE" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword ANALYZE not found.".into(),
            ))),
        }
    }
}

impl Display for Analyze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ANALYZE")
    }
}

impl SqliteKeyword for Analyze {}
