use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Vacuum;

impl FromStr for Vacuum {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "VACUUM" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword VACUUM not found.".into(),
            ))),
        }
    }
}

impl Display for Vacuum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "VACUUM")
    }
}

impl SqliteKeyword for Vacuum {}
