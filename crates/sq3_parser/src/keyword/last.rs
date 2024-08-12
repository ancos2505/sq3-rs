use std::{any::Any, fmt::Display};

use crate::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Last;
impl Last {
    pub const fn as_str() -> &'static str {
        "LAST"
    }
}

impl PartialEq<&str> for Last {
    fn eq(&self, other: &&str) -> bool {
        Last::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Last> for &str {
    fn eq(&self, _: &Last) -> bool {
        Last::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Last {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Last {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
