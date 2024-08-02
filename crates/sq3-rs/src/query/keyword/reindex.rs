use std::{any::Any, fmt::Display};

use crate::query::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Reindex;
impl Reindex {
    pub const fn as_str() -> &'static str {
        "REINDEX"
    }
}

impl PartialEq<&str> for Reindex {
    fn eq(&self, other: &&str) -> bool {
        Reindex::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Reindex> for &str {
    fn eq(&self, _: &Reindex) -> bool {
        Reindex::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Reindex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Reindex {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
