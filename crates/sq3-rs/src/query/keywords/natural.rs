use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Natural;
impl Natural {
    pub const fn as_str() -> &'static str {
        "NATURAL"
    }
}

impl PartialEq<&str> for Natural {
    fn eq(&self, other: &&str) -> bool {
        Natural::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Natural> for &str {
    fn eq(&self, _: &Natural) -> bool {
        Natural::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Natural {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Natural {}
