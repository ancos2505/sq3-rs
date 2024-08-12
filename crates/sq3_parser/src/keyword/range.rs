use std::{any::Any, fmt::Display};

use crate::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Range;
impl Range {
    pub const fn as_str() -> &'static str {
        "RANGE"
    }
}

impl PartialEq<&str> for Range {
    fn eq(&self, other: &&str) -> bool {
        Range::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Range> for &str {
    fn eq(&self, _: &Range) -> bool {
        Range::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Range {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
