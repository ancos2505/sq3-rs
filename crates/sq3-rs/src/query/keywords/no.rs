use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct No;
impl No {
    pub const fn as_str() -> &'static str {
        "NO"
    }
}

impl PartialEq<&str> for No {
    fn eq(&self, other: &&str) -> bool {
        No::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<No> for &str {
    fn eq(&self, _: &No) -> bool {
        No::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for No {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for No {}
