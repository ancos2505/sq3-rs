use std::{any::Any, fmt::Display};

use crate::query::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Group;
impl Group {
    pub const fn as_str() -> &'static str {
        "GROUP"
    }
}

impl PartialEq<&str> for Group {
    fn eq(&self, other: &&str) -> bool {
        Group::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Group> for &str {
    fn eq(&self, _: &Group) -> bool {
        Group::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Group {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Group {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
