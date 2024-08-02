use std::{any::Any, fmt::Display};

use crate::query::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Recursive;
impl Recursive {
    pub const fn as_str() -> &'static str {
        "RECURSIVE"
    }
}

impl PartialEq<&str> for Recursive {
    fn eq(&self, other: &&str) -> bool {
        Recursive::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Recursive> for &str {
    fn eq(&self, _: &Recursive) -> bool {
        Recursive::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Recursive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Recursive {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
