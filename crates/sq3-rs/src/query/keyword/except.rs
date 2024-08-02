use std::{any::Any, fmt::Display};

use crate::query::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Except;
impl Except {
    pub const fn as_str() -> &'static str {
        "EXCEPT"
    }
}

impl PartialEq<&str> for Except {
    fn eq(&self, other: &&str) -> bool {
        Except::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Except> for &str {
    fn eq(&self, _: &Except) -> bool {
        Except::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Except {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Except {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
