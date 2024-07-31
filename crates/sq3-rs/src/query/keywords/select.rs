use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Select;
impl Select {
    pub const fn as_str() -> &'static str {
        "SELECT"
    }
}

impl PartialEq<&str> for Select {
    fn eq(&self, other: &&str) -> bool {
        Select::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Select> for &str {
    fn eq(&self, _: &Select) -> bool {
        Select::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Select {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Select {}
