use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Where;
impl Where {
    pub const fn as_str() -> &'static str {
        "WHERE"
    }
}

impl PartialEq<&str> for Where {
    fn eq(&self, other: &&str) -> bool {
        Where::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Where> for &str {
    fn eq(&self, _: &Where) -> bool {
        Where::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Where {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Where {}
