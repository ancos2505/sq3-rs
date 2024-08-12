use std::{any::Any, fmt::Display};

use crate::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Then;
impl Then {
    pub const fn as_str() -> &'static str {
        "THEN"
    }
}

impl PartialEq<&str> for Then {
    fn eq(&self, other: &&str) -> bool {
        Then::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Then> for &str {
    fn eq(&self, _: &Then) -> bool {
        Then::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Then {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Then {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
