use std::{any::Any, fmt::Display};

use crate::query::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Ties;
impl Ties {
    pub const fn as_str() -> &'static str {
        "TIES"
    }
}

impl PartialEq<&str> for Ties {
    fn eq(&self, other: &&str) -> bool {
        Ties::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Ties> for &str {
    fn eq(&self, _: &Ties) -> bool {
        Ties::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Ties {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Ties {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
