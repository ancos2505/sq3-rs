use std::{any::Any, fmt::Display};

use crate::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Natural;
impl Natural {
    pub const fn as_str() -> &'static str {
        "NATURAL"
    }
}

impl PartialEq<&str> for Natural {
    fn eq(&self, other: &&str) -> bool {
        Natural::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Natural> for &str {
    fn eq(&self, _: &Natural) -> bool {
        Natural::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Natural {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Natural {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
