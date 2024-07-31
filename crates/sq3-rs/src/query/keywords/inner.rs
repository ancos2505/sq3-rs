use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Inner;
impl Inner {
    pub const fn as_str() -> &'static str {
        "INNER"
    }
}

impl PartialEq<&str> for Inner {
    fn eq(&self, other: &&str) -> bool {
        Inner::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Inner> for &str {
    fn eq(&self, _: &Inner) -> bool {
        Inner::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Inner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Inner {}
