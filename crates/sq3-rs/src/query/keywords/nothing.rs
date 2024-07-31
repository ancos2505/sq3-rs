use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Nothing;
impl Nothing {
    pub const fn as_str() -> &'static str {
        "NOTHING"
    }
}

impl PartialEq<&str> for Nothing {
    fn eq(&self, other: &&str) -> bool {
        Nothing::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Nothing> for &str {
    fn eq(&self, _: &Nothing) -> bool {
        Nothing::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Nothing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Nothing {}
