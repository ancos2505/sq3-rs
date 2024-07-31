use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Row;
impl Row {
    pub const fn as_str() -> &'static str {
        "ROW"
    }
}

impl PartialEq<&str> for Row {
    fn eq(&self, other: &&str) -> bool {
        Row::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Row> for &str {
    fn eq(&self, _: &Row) -> bool {
        Row::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Row {}
