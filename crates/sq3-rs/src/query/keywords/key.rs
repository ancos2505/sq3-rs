use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Key;
impl Key {
    pub const fn as_str() -> &'static str {
        "KEY"
    }
}

impl PartialEq<&str> for Key {
    fn eq(&self, other: &&str) -> bool {
        Key::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Key> for &str {
    fn eq(&self, _: &Key) -> bool {
        Key::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Key {}
