use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Release;

impl FromStr for Release {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "RELEASE" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword RELEASE not found.".into(),
            ))),
        }
    }
}

impl Display for Release {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RELEASE")
    }
}

impl SqliteKeyword for Release {}
