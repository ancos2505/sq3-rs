use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Detach;
impl Detach {
    pub const fn as_str() -> &'static str {
        "DETACH"
    }
}

impl PartialEq<&str> for Detach {
    fn eq(&self, other: &&str) -> bool {
        Detach::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Detach> for &str {
    fn eq(&self, _: &Detach) -> bool {
        Detach::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Detach {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Detach {}
