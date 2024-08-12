use std::{any::Any, fmt::Display};

use crate::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Ignore;
impl Ignore {
    pub const fn as_str() -> &'static str {
        "IGNORE"
    }
}

impl PartialEq<&str> for Ignore {
    fn eq(&self, other: &&str) -> bool {
        Ignore::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Ignore> for &str {
    fn eq(&self, _: &Ignore) -> bool {
        Ignore::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Ignore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Ignore {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
