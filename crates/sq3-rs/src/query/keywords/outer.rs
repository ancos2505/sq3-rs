use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Outer;
impl Outer {
    pub const fn as_str() -> &'static str {
        "OUTER"
    }
}

impl PartialEq<&str> for Outer {
    fn eq(&self, other: &&str) -> bool {
        Outer::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Outer> for &str {
    fn eq(&self, _: &Outer) -> bool {
        Outer::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Outer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Outer {}
