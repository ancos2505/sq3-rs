use std::{any::Any, fmt::Display};

use crate::query::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct For;
impl For {
    pub const fn as_str() -> &'static str {
        "FOR"
    }
}

impl PartialEq<&str> for For {
    fn eq(&self, other: &&str) -> bool {
        For::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<For> for &str {
    fn eq(&self, _: &For) -> bool {
        For::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for For {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for For {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
