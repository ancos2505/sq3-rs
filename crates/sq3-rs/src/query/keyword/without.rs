use std::{any::Any, fmt::Display};

use crate::query::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Without;
impl Without {
    pub const fn as_str() -> &'static str {
        "WITHOUT"
    }
}

impl PartialEq<&str> for Without {
    fn eq(&self, other: &&str) -> bool {
        Without::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Without> for &str {
    fn eq(&self, _: &Without) -> bool {
        Without::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Without {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Without {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
