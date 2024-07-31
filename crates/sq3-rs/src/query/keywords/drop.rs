use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Drop;
impl Drop {
    pub const fn as_str() -> &'static str {
        "DROP"
    }
}

impl PartialEq<&str> for Drop {
    fn eq(&self, other: &&str) -> bool {
        Drop::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Drop> for &str {
    fn eq(&self, _: &Drop) -> bool {
        Drop::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Drop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Drop {}
