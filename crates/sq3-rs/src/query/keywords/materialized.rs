use std::{any::Any, fmt::Display};

use crate::query::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Materialized;
impl Materialized {
    pub const fn as_str() -> &'static str {
        "MATERIALIZED"
    }
}

impl PartialEq<&str> for Materialized {
    fn eq(&self, other: &&str) -> bool {
        Materialized::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Materialized> for &str {
    fn eq(&self, _: &Materialized) -> bool {
        Materialized::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Materialized {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Materialized {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
