use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct By;
impl By {
    pub const fn as_str() -> &'static str {
        "BY"
    }
}

impl PartialEq<&str> for By {
    fn eq(&self, other: &&str) -> bool {
        By::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<By> for &str {
    fn eq(&self, _: &By) -> bool {
        By::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for By {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for By {}
