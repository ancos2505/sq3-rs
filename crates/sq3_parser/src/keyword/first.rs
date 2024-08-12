use std::{any::Any, fmt::Display};

use crate::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct First;
impl First {
    pub const fn as_str() -> &'static str {
        "FIRST"
    }
}

impl PartialEq<&str> for First {
    fn eq(&self, other: &&str) -> bool {
        First::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<First> for &str {
    fn eq(&self, _: &First) -> bool {
        First::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for First {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for First {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
