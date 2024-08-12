use std::{any::Any, fmt::Display};

use crate::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Primary;
impl Primary {
    pub const fn as_str() -> &'static str {
        "PRIMARY"
    }
}

impl PartialEq<&str> for Primary {
    fn eq(&self, other: &&str) -> bool {
        Primary::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Primary> for &str {
    fn eq(&self, _: &Primary) -> bool {
        Primary::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Primary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Primary {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
