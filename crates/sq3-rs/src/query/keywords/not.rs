use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Not;
impl Not {
    pub const fn as_str() -> &'static str {
        "NOT"
    }
}

impl PartialEq<&str> for Not {
    fn eq(&self, other: &&str) -> bool {
        Not::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Not> for &str {
    fn eq(&self, _: &Not) -> bool {
        Not::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Not {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Not {}
