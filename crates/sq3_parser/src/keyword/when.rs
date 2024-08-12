use std::{any::Any, fmt::Display};

use crate::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct When;
impl When {
    pub const fn as_str() -> &'static str {
        "WHEN"
    }
}

impl PartialEq<&str> for When {
    fn eq(&self, other: &&str) -> bool {
        When::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<When> for &str {
    fn eq(&self, _: &When) -> bool {
        When::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for When {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for When {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
