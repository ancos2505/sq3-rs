use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Like;
impl Like {
    pub const fn as_str() -> &'static str {
        "LIKE"
    }
}

impl PartialEq<&str> for Like {
    fn eq(&self, other: &&str) -> bool {
        Like::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Like> for &str {
    fn eq(&self, _: &Like) -> bool {
        Like::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Like {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Like {}
