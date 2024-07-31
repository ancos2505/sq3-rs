use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Cross;
impl Cross {
    pub const fn as_str() -> &'static str {
        "CROSS"
    }
}

impl PartialEq<&str> for Cross {
    fn eq(&self, other: &&str) -> bool {
        Cross::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Cross> for &str {
    fn eq(&self, _: &Cross) -> bool {
        Cross::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Cross {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Cross {}
