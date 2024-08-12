use std::{any::Any, fmt::Display};

use crate::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
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

impl SqliteKeyword for Key {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
