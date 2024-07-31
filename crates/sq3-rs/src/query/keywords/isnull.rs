use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Isnull;
impl Isnull {
    pub const fn as_str() -> &'static str {
        "ISNULL"
    }
}

impl PartialEq<&str> for Isnull {
    fn eq(&self, other: &&str) -> bool {
        Isnull::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Isnull> for &str {
    fn eq(&self, _: &Isnull) -> bool {
        Isnull::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Isnull {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Isnull {}
