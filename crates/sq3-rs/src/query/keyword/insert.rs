use std::{any::Any, fmt::Display};

use crate::query::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Insert;
impl Insert {
    pub const fn as_str() -> &'static str {
        "INSERT"
    }
}

impl PartialEq<&str> for Insert {
    fn eq(&self, other: &&str) -> bool {
        Insert::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Insert> for &str {
    fn eq(&self, _: &Insert) -> bool {
        Insert::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Insert {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Insert {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
