use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Then;
impl Then {
    pub const fn as_str() -> &'static str {
        "THEN"
    }
}

impl PartialEq<&str> for Then {
    fn eq(&self, other: &&str) -> bool {
        Then::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Then> for &str {
    fn eq(&self, _: &Then) -> bool {
        Then::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Then {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Then {}
