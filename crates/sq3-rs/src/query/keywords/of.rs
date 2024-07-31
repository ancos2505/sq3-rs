use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Of;
impl Of {
    pub const fn as_str() -> &'static str {
        "OF"
    }
}

impl PartialEq<&str> for Of {
    fn eq(&self, other: &&str) -> bool {
        Of::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Of> for &str {
    fn eq(&self, _: &Of) -> bool {
        Of::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Of {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Of {}
