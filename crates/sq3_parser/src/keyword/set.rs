use std::{any::Any, fmt::Display};

use crate::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Set;
impl Set {
    pub const fn as_str() -> &'static str {
        "SET"
    }
}

impl PartialEq<&str> for Set {
    fn eq(&self, other: &&str) -> bool {
        Set::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Set> for &str {
    fn eq(&self, _: &Set) -> bool {
        Set::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Set {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Set {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
