use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Using;
impl Using {
    pub const fn as_str() -> &'static str {
        "USING"
    }
}

impl PartialEq<&str> for Using {
    fn eq(&self, other: &&str) -> bool {
        Using::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Using> for &str {
    fn eq(&self, _: &Using) -> bool {
        Using::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Using {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Using {}
