use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Returning;
impl Returning {
    pub const fn as_str() -> &'static str {
        "RETURNING"
    }
}

impl PartialEq<&str> for Returning {
    fn eq(&self, other: &&str) -> bool {
        Returning::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Returning> for &str {
    fn eq(&self, _: &Returning) -> bool {
        Returning::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Returning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Returning {}
