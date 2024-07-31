use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Null;
impl Null {
    pub const fn as_str() -> &'static str {
        "NULL"
    }
}

impl PartialEq<&str> for Null {
    fn eq(&self, other: &&str) -> bool {
        Null::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Null> for &str {
    fn eq(&self, _: &Null) -> bool {
        Null::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Null {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Null {}
