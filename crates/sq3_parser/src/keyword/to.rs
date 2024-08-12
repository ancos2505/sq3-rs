use std::{any::Any, fmt::Display};

use crate::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct To;
impl To {
    pub const fn as_str() -> &'static str {
        "TO"
    }
}

impl PartialEq<&str> for To {
    fn eq(&self, other: &&str) -> bool {
        To::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<To> for &str {
    fn eq(&self, _: &To) -> bool {
        To::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for To {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for To {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
