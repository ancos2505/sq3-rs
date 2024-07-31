use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Left;
impl Left {
    pub const fn as_str() -> &'static str {
        "LEFT"
    }
}

impl PartialEq<&str> for Left {
    fn eq(&self, other: &&str) -> bool {
        Left::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Left> for &str {
    fn eq(&self, _: &Left) -> bool {
        Left::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Left {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Left {}
