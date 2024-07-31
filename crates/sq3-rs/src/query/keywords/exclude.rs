use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Exclude;
impl Exclude {
    pub const fn as_str() -> &'static str {
        "EXCLUDE"
    }
}

impl PartialEq<&str> for Exclude {
    fn eq(&self, other: &&str) -> bool {
        Exclude::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Exclude> for &str {
    fn eq(&self, _: &Exclude) -> bool {
        Exclude::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Exclude {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Exclude {}
