use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Exists;
impl Exists {
    pub const fn as_str() -> &'static str {
        "EXISTS"
    }
}

impl PartialEq<&str> for Exists {
    fn eq(&self, other: &&str) -> bool {
        Exists::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Exists> for &str {
    fn eq(&self, _: &Exists) -> bool {
        Exists::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Exists {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Exists {}
