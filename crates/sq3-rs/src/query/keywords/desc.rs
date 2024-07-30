use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Desc;

impl FromStr for Desc {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DESC" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword DESC not found.".into(),
            ))),
        }
    }
}

impl Display for Desc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DESC")
    }
}

impl SqliteKeyword for Desc {}
