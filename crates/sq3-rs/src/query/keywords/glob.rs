use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Glob;
impl Glob {
    pub const fn as_str() -> &'static str {
        "GLOB"
    }
}

impl PartialEq<&str> for Glob {
    fn eq(&self, other: &&str) -> bool {
        Glob::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Glob> for &str {
    fn eq(&self, _: &Glob) -> bool {
        Glob::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Glob {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Glob {}
