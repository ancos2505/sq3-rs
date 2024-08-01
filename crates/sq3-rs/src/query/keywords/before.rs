use std::{any::Any, fmt::Display};

use crate::query::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Before;
impl Before {
    pub const fn as_str() -> &'static str {
        "BEFORE"
    }
}

impl PartialEq<&str> for Before {
    fn eq(&self, other: &&str) -> bool {
        Before::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Before> for &str {
    fn eq(&self, _: &Before) -> bool {
        Before::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Before {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Before {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
