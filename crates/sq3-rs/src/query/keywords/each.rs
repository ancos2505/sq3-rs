use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Each;
impl Each {
    pub const fn as_str() -> &'static str {
        "EACH"
    }
}

impl PartialEq<&str> for Each {
    fn eq(&self, other: &&str) -> bool {
        Each::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Each> for &str {
    fn eq(&self, _: &Each) -> bool {
        Each::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Each {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Each {}
