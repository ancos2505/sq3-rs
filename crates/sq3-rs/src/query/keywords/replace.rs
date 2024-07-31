use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Replace;
impl Replace {
    pub const fn as_str() -> &'static str {
        "REPLACE"
    }
}

impl PartialEq<&str> for Replace {
    fn eq(&self, other: &&str) -> bool {
        Replace::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Replace> for &str {
    fn eq(&self, _: &Replace) -> bool {
        Replace::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Replace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Replace {}
