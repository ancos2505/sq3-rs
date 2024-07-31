use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Pragma;
impl Pragma {
    pub const fn as_str() -> &'static str {
        "PRAGMA"
    }
}

impl PartialEq<&str> for Pragma {
    fn eq(&self, other: &&str) -> bool {
        Pragma::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Pragma> for &str {
    fn eq(&self, _: &Pragma) -> bool {
        Pragma::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Pragma {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Pragma {}
