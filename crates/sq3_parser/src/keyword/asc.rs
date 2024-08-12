use std::{any::Any, fmt::Display};

use crate::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Asc;
impl Asc {
    pub const fn as_str() -> &'static str {
        "ASC"
    }
}

impl PartialEq<&str> for Asc {
    fn eq(&self, other: &&str) -> bool {
        Asc::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Asc> for &str {
    fn eq(&self, _: &Asc) -> bool {
        Asc::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Asc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Asc {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
