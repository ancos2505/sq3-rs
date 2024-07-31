use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Abort;
impl Abort {
    pub const fn as_str() -> &'static str {
        "ABORT"
    }
}

impl PartialEq<&str> for Abort {
    fn eq(&self, other: &&str) -> bool {
        Abort::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Abort> for &str {
    fn eq(&self, _: &Abort) -> bool {
        Abort::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Abort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Abort {}
