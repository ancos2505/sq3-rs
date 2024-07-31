use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Or;
impl Or {
    pub const fn as_str() -> &'static str {
        "OR"
    }
}

impl PartialEq<&str> for Or {
    fn eq(&self, other: &&str) -> bool {
        Or::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Or> for &str {
    fn eq(&self, _: &Or) -> bool {
        Or::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Or {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Or {}
