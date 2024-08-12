use std::{any::Any, fmt::Display};

use crate::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Unique;
impl Unique {
    pub const fn as_str() -> &'static str {
        "UNIQUE"
    }
}

impl PartialEq<&str> for Unique {
    fn eq(&self, other: &&str) -> bool {
        Unique::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Unique> for &str {
    fn eq(&self, _: &Unique) -> bool {
        Unique::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Unique {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Unique {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
