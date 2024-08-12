use std::{any::Any, fmt::Display};

use crate::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct After;
impl After {
    pub const fn as_str() -> &'static str {
        "AFTER"
    }
}

impl PartialEq<&str> for After {
    fn eq(&self, other: &&str) -> bool {
        After::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<After> for &str {
    fn eq(&self, _: &After) -> bool {
        After::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for After {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for After {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
