use std::str::FromStr;

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(super) struct Default;

impl FromStr for Default {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DEFAULT" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(Box::new(Self)))),
        }
    }
}

impl SqliteKeyword for Default {}
