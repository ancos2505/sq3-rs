use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Explain;
impl Explain {
    pub const fn as_str() -> &'static str {
        "EXPLAIN"
    }
}

impl PartialEq<&str> for Explain {
    fn eq(&self, other: &&str) -> bool {
        Explain::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Explain> for &str {
    fn eq(&self, _: &Explain) -> bool {
        Explain::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Explain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Explain {}
