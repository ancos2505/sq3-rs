use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Deferrable;
impl Deferrable {
    pub const fn as_str() -> &'static str {
        "DEFERRABLE"
    }
}

impl PartialEq<&str> for Deferrable {
    fn eq(&self, other: &&str) -> bool {
        Deferrable::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Deferrable> for &str {
    fn eq(&self, _: &Deferrable) -> bool {
        Deferrable::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Deferrable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Deferrable {}
