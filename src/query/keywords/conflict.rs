use std::str::FromStr;

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(super) struct Conflict;

impl FromStr for Conflict {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CONFLICT" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(Box::new(Self)))),
        }
    }
}

impl SqliteKeyword for Conflict {}
