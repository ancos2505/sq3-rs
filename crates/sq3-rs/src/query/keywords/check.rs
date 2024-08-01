use std::{any::Any, fmt::Display};

use crate::query::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Check;
impl Check {
    pub const fn as_str() -> &'static str {
        "CHECK"
    }
}

impl PartialEq<&str> for Check {
    fn eq(&self, other: &&str) -> bool {
        Check::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Check> for &str {
    fn eq(&self, _: &Check) -> bool {
        Check::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Check {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Check {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
