use std::{any::Any, fmt::Display};

use crate::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Savepoint;
impl Savepoint {
    pub const fn as_str() -> &'static str {
        "SAVEPOINT"
    }
}

impl PartialEq<&str> for Savepoint {
    fn eq(&self, other: &&str) -> bool {
        Savepoint::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Savepoint> for &str {
    fn eq(&self, _: &Savepoint) -> bool {
        Savepoint::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Savepoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Savepoint {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
