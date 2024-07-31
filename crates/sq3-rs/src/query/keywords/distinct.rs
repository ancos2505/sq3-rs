use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Distinct;
impl Distinct {
    pub const fn as_str() -> &'static str {
        "DISTINCT"
    }
}

impl PartialEq<&str> for Distinct {
    fn eq(&self, other: &&str) -> bool {
        Distinct::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Distinct> for &str {
    fn eq(&self, _: &Distinct) -> bool {
        Distinct::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Distinct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Distinct {}
