use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct All;
impl All {
    pub const fn as_str() -> &'static str {
        "ALL"
    }
}

impl PartialEq<&str> for All {
    fn eq(&self, other: &&str) -> bool {
        All::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<All> for &str {
    fn eq(&self, _: &All) -> bool {
        All::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for All {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for All {}
