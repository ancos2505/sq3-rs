use std::{any::Any, fmt::Display};

use crate::query::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Between;
impl Between {
    pub const fn as_str() -> &'static str {
        "BETWEEN"
    }
}

impl PartialEq<&str> for Between {
    fn eq(&self, other: &&str) -> bool {
        Between::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Between> for &str {
    fn eq(&self, _: &Between) -> bool {
        Between::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Between {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Between {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
