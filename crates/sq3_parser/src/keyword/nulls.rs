use std::{any::Any, fmt::Display};

use crate::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Nulls;
impl Nulls {
    pub const fn as_str() -> &'static str {
        "NULLS"
    }
}

impl PartialEq<&str> for Nulls {
    fn eq(&self, other: &&str) -> bool {
        Nulls::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Nulls> for &str {
    fn eq(&self, _: &Nulls) -> bool {
        Nulls::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Nulls {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Nulls {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
