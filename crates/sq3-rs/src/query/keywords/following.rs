use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Following;
impl Following {
    pub const fn as_str() -> &'static str {
        "FOLLOWING"
    }
}

impl PartialEq<&str> for Following {
    fn eq(&self, other: &&str) -> bool {
        Following::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Following> for &str {
    fn eq(&self, _: &Following) -> bool {
        Following::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Following {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Following {}
