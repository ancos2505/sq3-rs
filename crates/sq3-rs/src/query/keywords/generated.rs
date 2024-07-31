use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Generated;
impl Generated {
    pub const fn as_str() -> &'static str {
        "GENERATED"
    }
}

impl PartialEq<&str> for Generated {
    fn eq(&self, other: &&str) -> bool {
        Generated::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Generated> for &str {
    fn eq(&self, _: &Generated) -> bool {
        Generated::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Generated {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Generated {}
