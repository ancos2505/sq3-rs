use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Union;
impl Union {
    pub const fn as_str() -> &'static str {
        "UNION"
    }
}

impl PartialEq<&str> for Union {
    fn eq(&self, other: &&str) -> bool {
        Union::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Union> for &str {
    fn eq(&self, _: &Union) -> bool {
        Union::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Union {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Union {}
