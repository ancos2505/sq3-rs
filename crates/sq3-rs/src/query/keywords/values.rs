use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Values;
impl Values {
    pub const fn as_str() -> &'static str {
        "VALUES"
    }
}

impl PartialEq<&str> for Values {
    fn eq(&self, other: &&str) -> bool {
        Values::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Values> for &str {
    fn eq(&self, _: &Values) -> bool {
        Values::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Values {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Values {}
