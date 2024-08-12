use std::{any::Any, fmt::Display};

use crate::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Begin;
impl Begin {
    pub const fn as_str() -> &'static str {
        "BEGIN"
    }
}

impl PartialEq<&str> for Begin {
    fn eq(&self, other: &&str) -> bool {
        Begin::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Begin> for &str {
    fn eq(&self, _: &Begin) -> bool {
        Begin::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Begin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Begin {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
