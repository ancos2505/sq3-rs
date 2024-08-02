use std::{any::Any, fmt::Display};

use crate::query::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Update;
impl Update {
    pub const fn as_str() -> &'static str {
        "UPDATE"
    }
}

impl PartialEq<&str> for Update {
    fn eq(&self, other: &&str) -> bool {
        Update::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Update> for &str {
    fn eq(&self, _: &Update) -> bool {
        Update::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Update {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Update {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
