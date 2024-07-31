use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct With;
impl With {
    pub const fn as_str() -> &'static str {
        "WITH"
    }
}

impl PartialEq<&str> for With {
    fn eq(&self, other: &&str) -> bool {
        With::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<With> for &str {
    fn eq(&self, _: &With) -> bool {
        With::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for With {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for With {}
