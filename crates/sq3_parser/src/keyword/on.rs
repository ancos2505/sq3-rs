use std::{any::Any, fmt::Display};

use crate::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct On;
impl On {
    pub const fn as_str() -> &'static str {
        "ON"
    }
}

impl PartialEq<&str> for On {
    fn eq(&self, other: &&str) -> bool {
        On::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<On> for &str {
    fn eq(&self, _: &On) -> bool {
        On::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for On {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for On {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
