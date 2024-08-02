use std::{any::Any, fmt::Display};

use crate::query::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Conflict;
impl Conflict {
    pub const fn as_str() -> &'static str {
        "CONFLICT"
    }
}

impl PartialEq<&str> for Conflict {
    fn eq(&self, other: &&str) -> bool {
        Conflict::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Conflict> for &str {
    fn eq(&self, _: &Conflict) -> bool {
        Conflict::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Conflict {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Conflict {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
