use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Instead;
impl Instead {
    pub const fn as_str() -> &'static str {
        "INSTEAD"
    }
}

impl PartialEq<&str> for Instead {
    fn eq(&self, other: &&str) -> bool {
        Instead::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Instead> for &str {
    fn eq(&self, _: &Instead) -> bool {
        Instead::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Instead {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Instead {}
