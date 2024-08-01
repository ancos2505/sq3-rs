use std::{any::Any, fmt::Display};

use crate::query::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Index;
impl Index {
    pub const fn as_str() -> &'static str {
        "INDEX"
    }
}

impl PartialEq<&str> for Index {
    fn eq(&self, other: &&str) -> bool {
        Index::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Index> for &str {
    fn eq(&self, _: &Index) -> bool {
        Index::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Index {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Index {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
