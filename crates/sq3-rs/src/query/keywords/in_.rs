use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct In;
impl In {
    pub const fn as_str() -> &'static str {
        "IN"
    }
}

impl PartialEq<&str> for In {
    fn eq(&self, other: &&str) -> bool {
        In::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<In> for &str {
    fn eq(&self, _: &In) -> bool {
        In::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for In {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for In {}
