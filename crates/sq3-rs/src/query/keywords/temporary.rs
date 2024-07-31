use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Temporary;
impl Temporary {
    pub const fn as_str() -> &'static str {
        "TEMPORARY"
    }
}

impl PartialEq<&str> for Temporary {
    fn eq(&self, other: &&str) -> bool {
        Temporary::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Temporary> for &str {
    fn eq(&self, _: &Temporary) -> bool {
        Temporary::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Temporary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Temporary {}
