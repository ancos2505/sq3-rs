use std::{any::Any, fmt::Display};

use crate::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Full;
impl Full {
    pub const fn as_str() -> &'static str {
        "FULL"
    }
}

impl PartialEq<&str> for Full {
    fn eq(&self, other: &&str) -> bool {
        Full::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Full> for &str {
    fn eq(&self, _: &Full) -> bool {
        Full::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Full {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Full {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
