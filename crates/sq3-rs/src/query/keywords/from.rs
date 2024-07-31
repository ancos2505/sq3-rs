use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct From;
impl From {
    pub const fn as_str() -> &'static str {
        "FROM"
    }
}

impl PartialEq<&str> for From {
    fn eq(&self, other: &&str) -> bool {
        From::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<From> for &str {
    fn eq(&self, _: &From) -> bool {
        From::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for From {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for From {}
