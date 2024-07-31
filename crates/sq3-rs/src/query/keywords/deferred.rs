use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Deferred;
impl Deferred {
    pub const fn as_str() -> &'static str {
        "DEFERRED"
    }
}

impl PartialEq<&str> for Deferred {
    fn eq(&self, other: &&str) -> bool {
        Deferred::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Deferred> for &str {
    fn eq(&self, _: &Deferred) -> bool {
        Deferred::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Deferred {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Deferred {}
