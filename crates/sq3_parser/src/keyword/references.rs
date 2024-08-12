use std::{any::Any, fmt::Display};

use crate::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct References;
impl References {
    pub const fn as_str() -> &'static str {
        "REFERENCES"
    }
}

impl PartialEq<&str> for References {
    fn eq(&self, other: &&str) -> bool {
        References::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<References> for &str {
    fn eq(&self, _: &References) -> bool {
        References::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for References {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for References {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
