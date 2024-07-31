use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Without;
impl Without {
    pub const fn as_str() -> &'static str {
        "WITHOUT"
    }
}

impl PartialEq<&str> for Without {
    fn eq(&self, other: &&str) -> bool {
        Without::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Without> for &str {
    fn eq(&self, _: &Without) -> bool {
        Without::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Without {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Without {}
