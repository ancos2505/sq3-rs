use std::{any::Any, fmt::Display};

use crate::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Join;
impl Join {
    pub const fn as_str() -> &'static str {
        "JOIN"
    }
}

impl PartialEq<&str> for Join {
    fn eq(&self, other: &&str) -> bool {
        Join::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Join> for &str {
    fn eq(&self, _: &Join) -> bool {
        Join::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Join {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Join {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
