use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct As;
impl As {
    pub const fn as_str() -> &'static str {
        "AS"
    }
}

impl PartialEq<&str> for As {
    fn eq(&self, other: &&str) -> bool {
        As::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<As> for &str {
    fn eq(&self, _: &As) -> bool {
        As::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for As {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for As {}
