use std::{any::Any, fmt::Display};

use crate::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Restrict;
impl Restrict {
    pub const fn as_str() -> &'static str {
        "RESTRICT"
    }
}

impl PartialEq<&str> for Restrict {
    fn eq(&self, other: &&str) -> bool {
        Restrict::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Restrict> for &str {
    fn eq(&self, _: &Restrict) -> bool {
        Restrict::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Restrict {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Restrict {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
