use std::{any::Any, fmt::Display};

use crate::query::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Case;
impl Case {
    pub const fn as_str() -> &'static str {
        "CASE"
    }
}

impl PartialEq<&str> for Case {
    fn eq(&self, other: &&str) -> bool {
        Case::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Case> for &str {
    fn eq(&self, _: &Case) -> bool {
        Case::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Case {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Case {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
