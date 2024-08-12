use std::{any::Any, fmt::Display};

use crate::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Offset;
impl Offset {
    pub const fn as_str() -> &'static str {
        "OFFSET"
    }
}

impl PartialEq<&str> for Offset {
    fn eq(&self, other: &&str) -> bool {
        Offset::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Offset> for &str {
    fn eq(&self, _: &Offset) -> bool {
        Offset::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Offset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Offset {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
