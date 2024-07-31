use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Is;
impl Is {
    pub const fn as_str() -> &'static str {
        "IS"
    }
}

impl PartialEq<&str> for Is {
    fn eq(&self, other: &&str) -> bool {
        Is::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Is> for &str {
    fn eq(&self, _: &Is) -> bool {
        Is::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Is {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Is {}
