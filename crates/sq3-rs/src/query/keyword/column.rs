use std::{any::Any, fmt::Display};

use crate::query::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Column;
impl Column {
    pub const fn as_str() -> &'static str {
        "COLUMN"
    }
}

impl PartialEq<&str> for Column {
    fn eq(&self, other: &&str) -> bool {
        Column::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Column> for &str {
    fn eq(&self, _: &Column) -> bool {
        Column::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Column {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Column {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
