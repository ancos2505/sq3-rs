use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Others;
impl Others {
    pub const fn as_str() -> &'static str {
        "OTHERS"
    }
}

impl PartialEq<&str> for Others {
    fn eq(&self, other: &&str) -> bool {
        Others::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Others> for &str {
    fn eq(&self, _: &Others) -> bool {
        Others::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Others {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Others {}
