use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Create;

impl FromStr for Create {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CREATE" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword CREATE not found.".into(),
            ))),
        }
    }
}

impl Display for Create {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CREATE")
    }
}

impl SqliteKeyword for Create {}
