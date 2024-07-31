use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Join;
impl Join {
    pub const fn as_str() -> &'static str {
        "JOIN"
    }
}

impl PartialEq<&str> for Join {
    fn eq(&self, other: &&str) -> bool {
        Join::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Join> for &str {
    fn eq(&self, _: &Join) -> bool {
        Join::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Join {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Join {}
