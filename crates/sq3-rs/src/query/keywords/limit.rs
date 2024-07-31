use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Limit;
impl Limit {
    pub const fn as_str() -> &'static str {
        "LIMIT"
    }
}

impl PartialEq<&str> for Limit {
    fn eq(&self, other: &&str) -> bool {
        Limit::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Limit> for &str {
    fn eq(&self, _: &Limit) -> bool {
        Limit::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Limit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Limit {}
