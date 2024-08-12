use std::{any::Any, fmt::Display};

use crate::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Escape;
impl Escape {
    pub const fn as_str() -> &'static str {
        "ESCAPE"
    }
}

impl PartialEq<&str> for Escape {
    fn eq(&self, other: &&str) -> bool {
        Escape::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Escape> for &str {
    fn eq(&self, _: &Escape) -> bool {
        Escape::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Escape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Escape {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
