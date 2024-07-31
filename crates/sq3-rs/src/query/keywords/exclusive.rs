use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Exclusive;
impl Exclusive {
    pub const fn as_str() -> &'static str {
        "EXCLUSIVE"
    }
}

impl PartialEq<&str> for Exclusive {
    fn eq(&self, other: &&str) -> bool {
        Exclusive::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Exclusive> for &str {
    fn eq(&self, _: &Exclusive) -> bool {
        Exclusive::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Exclusive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Exclusive {}
