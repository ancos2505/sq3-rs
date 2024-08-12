use std::{any::Any, fmt::Display};

use crate::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Database;
impl Database {
    pub const fn as_str() -> &'static str {
        "DATABASE"
    }
}

impl PartialEq<&str> for Database {
    fn eq(&self, other: &&str) -> bool {
        Database::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Database> for &str {
    fn eq(&self, _: &Database) -> bool {
        Database::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Database {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Database {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
