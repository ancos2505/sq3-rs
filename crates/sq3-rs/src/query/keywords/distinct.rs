use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::{DistinctProcessing, SqliteKeyword},
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Distinct;

impl FromStr for Distinct {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DISTINCT" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword DISTINCT not found.".into(),
            ))),
        }
    }
}

impl Display for Distinct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DISTINCT")
    }
}

impl SqliteKeyword for Distinct {}
impl DistinctProcessing for Distinct {}
