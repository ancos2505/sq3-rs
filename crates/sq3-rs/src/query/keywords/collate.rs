use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Collate;
impl Collate {
    pub const fn as_str() -> &'static str {
        "COLLATE"
    }
}

impl PartialEq<&str> for Collate {
    fn eq(&self, other: &&str) -> bool {
        Collate::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Collate> for &str {
    fn eq(&self, _: &Collate) -> bool {
        Collate::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Collate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Collate {}
