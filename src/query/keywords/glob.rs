use std::str::FromStr;

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(super) struct Glob;

impl FromStr for Glob {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GLOB" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(Box::new(Self)))),
        }
    }
}

impl SqliteKeyword for Glob {}
