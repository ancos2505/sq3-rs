use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Into;
impl Into {
    pub const fn as_str() -> &'static str {
        "INTO"
    }
}

impl PartialEq<&str> for Into {
    fn eq(&self, other: &&str) -> bool {
        Into::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Into> for &str {
    fn eq(&self, _: &Into) -> bool {
        Into::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Into {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Into {}
