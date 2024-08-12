use std::{any::Any, fmt::Display};

use crate::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Delete;
impl Delete {
    pub const fn as_str() -> &'static str {
        "DELETE"
    }
}

impl PartialEq<&str> for Delete {
    fn eq(&self, other: &&str) -> bool {
        Delete::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Delete> for &str {
    fn eq(&self, _: &Delete) -> bool {
        Delete::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Delete {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Delete {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
