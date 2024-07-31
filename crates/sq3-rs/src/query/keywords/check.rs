use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Check;
impl Check {
    pub const fn as_str() -> &'static str {
        "CHECK"
    }
}

impl PartialEq<&str> for Check {
    fn eq(&self, other: &&str) -> bool {
        Check::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Check> for &str {
    fn eq(&self, _: &Check) -> bool {
        Check::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Check {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Check {}
