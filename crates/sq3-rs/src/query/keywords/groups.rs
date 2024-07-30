use std::{fmt::Display, str::FromStr};

use crate::{
    query::traits::SqliteKeyword,
    result::{SqlParserError, SqliteError},
};

#[derive(Debug)]
pub(crate) struct Groups;

impl FromStr for Groups {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GROUPS" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword GROUPS not found.".into(),
            ))),
        }
    }
}

impl Display for Groups {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GROUPS")
    }
}

impl SqliteKeyword for Groups {}
