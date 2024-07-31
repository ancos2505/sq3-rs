use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Regexp;
impl Regexp {
    pub const fn as_str() -> &'static str {
        "REGEXP"
    }
}

impl PartialEq<&str> for Regexp {
    fn eq(&self, other: &&str) -> bool {
        Regexp::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Regexp> for &str {
    fn eq(&self, _: &Regexp) -> bool {
        Regexp::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Regexp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Regexp {}
