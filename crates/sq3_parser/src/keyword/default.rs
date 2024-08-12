use std::{any::Any, fmt::Display};

use crate::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Default;
impl Default {
    pub const fn as_str() -> &'static str {
        "DEFAULT"
    }
}

impl PartialEq<&str> for Default {
    fn eq(&self, other: &&str) -> bool {
        Default::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Default> for &str {
    fn eq(&self, _: &Default) -> bool {
        Default::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Default {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Default {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
