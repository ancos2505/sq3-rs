use std::{any::Any, fmt::Display};

use crate::query::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Temporary;
impl Temporary {
    pub const fn as_str() -> &'static str {
        "TEMPORARY"
    }
}

impl PartialEq<&str> for Temporary {
    fn eq(&self, other: &&str) -> bool {
        Temporary::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Temporary> for &str {
    fn eq(&self, _: &Temporary) -> bool {
        Temporary::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Temporary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Temporary {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
