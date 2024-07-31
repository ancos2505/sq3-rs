use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Having;
impl Having {
    pub const fn as_str() -> &'static str {
        "HAVING"
    }
}

impl PartialEq<&str> for Having {
    fn eq(&self, other: &&str) -> bool {
        Having::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Having> for &str {
    fn eq(&self, _: &Having) -> bool {
        Having::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Having {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Having {}
