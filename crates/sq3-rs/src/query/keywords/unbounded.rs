use std::{any::Any, fmt::Display};

use crate::query::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Unbounded;
impl Unbounded {
    pub const fn as_str() -> &'static str {
        "UNBOUNDED"
    }
}

impl PartialEq<&str> for Unbounded {
    fn eq(&self, other: &&str) -> bool {
        Unbounded::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Unbounded> for &str {
    fn eq(&self, _: &Unbounded) -> bool {
        Unbounded::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Unbounded {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Unbounded {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
