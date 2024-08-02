use std::{any::Any, fmt::Display};

use crate::query::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Partition;
impl Partition {
    pub const fn as_str() -> &'static str {
        "PARTITION"
    }
}

impl PartialEq<&str> for Partition {
    fn eq(&self, other: &&str) -> bool {
        Partition::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Partition> for &str {
    fn eq(&self, _: &Partition) -> bool {
        Partition::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Partition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Partition {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
