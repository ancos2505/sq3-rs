use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Default;
impl Default {
    pub const fn as_str() -> &'static str {
        "DEFAULT"
    }
}

impl PartialEq<&str> for Default {
    fn eq(&self, other: &&str) -> bool {
        Default::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Default> for &str {
    fn eq(&self, _: &Default) -> bool {
        Default::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Default {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Default {}
