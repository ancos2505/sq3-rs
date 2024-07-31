use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Add;
impl Add {
    pub const fn as_str() -> &'static str {
        "ADD"
    }
}

impl PartialEq<&str> for Add {
    fn eq(&self, other: &&str) -> bool {
        Add::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Add> for &str {
    fn eq(&self, _: &Add) -> bool {
        Add::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Add {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Add {}
