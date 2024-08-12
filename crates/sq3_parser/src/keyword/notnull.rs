use std::{any::Any, fmt::Display};

use crate::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Notnull;
impl Notnull {
    pub const fn as_str() -> &'static str {
        "NOTNULL"
    }
}

impl PartialEq<&str> for Notnull {
    fn eq(&self, other: &&str) -> bool {
        Notnull::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Notnull> for &str {
    fn eq(&self, _: &Notnull) -> bool {
        Notnull::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Notnull {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Notnull {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
