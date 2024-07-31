use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Cascade;
impl Cascade {
    pub const fn as_str() -> &'static str {
        "CASCADE"
    }
}

impl PartialEq<&str> for Cascade {
    fn eq(&self, other: &&str) -> bool {
        Cascade::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Cascade> for &str {
    fn eq(&self, _: &Cascade) -> bool {
        Cascade::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Cascade {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Cascade {}
