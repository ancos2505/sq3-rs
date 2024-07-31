use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Right;
impl Right {
    pub const fn as_str() -> &'static str {
        "RIGHT"
    }
}

impl PartialEq<&str> for Right {
    fn eq(&self, other: &&str) -> bool {
        Right::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Right> for &str {
    fn eq(&self, _: &Right) -> bool {
        Right::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Right {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Right {}
