use std::{any::Any, fmt::Display};

use crate::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
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

impl SqliteKeyword for Like {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
