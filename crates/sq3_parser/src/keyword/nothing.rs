use std::{any::Any, fmt::Display};

use crate::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Nothing;
impl Nothing {
    pub const fn as_str() -> &'static str {
        "NOTHING"
    }
}

impl PartialEq<&str> for Nothing {
    fn eq(&self, other: &&str) -> bool {
        Nothing::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Nothing> for &str {
    fn eq(&self, _: &Nothing) -> bool {
        Nothing::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Nothing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Nothing {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
