use std::{any::Any, fmt::Display};

use crate::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Current;
impl Current {
    pub const fn as_str() -> &'static str {
        "CURRENT"
    }
}

impl PartialEq<&str> for Current {
    fn eq(&self, other: &&str) -> bool {
        Current::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Current> for &str {
    fn eq(&self, _: &Current) -> bool {
        Current::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Current {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Current {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
