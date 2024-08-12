use std::{any::Any, fmt::Display};

use crate::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Immediate;
impl Immediate {
    pub const fn as_str() -> &'static str {
        "IMMEDIATE"
    }
}

impl PartialEq<&str> for Immediate {
    fn eq(&self, other: &&str) -> bool {
        Immediate::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Immediate> for &str {
    fn eq(&self, _: &Immediate) -> bool {
        Immediate::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Immediate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Immediate {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
