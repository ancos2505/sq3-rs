use std::{any::Any, fmt::Display};

use crate::query::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Table;
impl Table {
    pub const fn as_str() -> &'static str {
        "TABLE"
    }
}

impl PartialEq<&str> for Table {
    fn eq(&self, other: &&str) -> bool {
        Table::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Table> for &str {
    fn eq(&self, _: &Table) -> bool {
        Table::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Table {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
