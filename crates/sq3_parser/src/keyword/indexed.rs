use std::{any::Any, fmt::Display};

use crate::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Indexed;
impl Indexed {
    pub const fn as_str() -> &'static str {
        "INDEXED"
    }
}

impl PartialEq<&str> for Indexed {
    fn eq(&self, other: &&str) -> bool {
        Indexed::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Indexed> for &str {
    fn eq(&self, _: &Indexed) -> bool {
        Indexed::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Indexed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Indexed {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
