use std::{any::Any, fmt::Display};

use crate::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Always;
impl Always {
    pub const fn as_str() -> &'static str {
        "ALWAYS"
    }
}

impl PartialEq<&str> for Always {
    fn eq(&self, other: &&str) -> bool {
        Always::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Always> for &str {
    fn eq(&self, _: &Always) -> bool {
        Always::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Always {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Always {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
