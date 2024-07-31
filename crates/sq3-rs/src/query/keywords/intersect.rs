use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Intersect;
impl Intersect {
    pub const fn as_str() -> &'static str {
        "INTERSECT"
    }
}

impl PartialEq<&str> for Intersect {
    fn eq(&self, other: &&str) -> bool {
        Intersect::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Intersect> for &str {
    fn eq(&self, _: &Intersect) -> bool {
        Intersect::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Intersect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Intersect {}
