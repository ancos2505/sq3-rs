use std::{any::Any, fmt::Display};

use crate::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Union;
impl Union {
    pub const fn as_str() -> &'static str {
        "UNION"
    }
}

impl PartialEq<&str> for Union {
    fn eq(&self, other: &&str) -> bool {
        Union::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Union> for &str {
    fn eq(&self, _: &Union) -> bool {
        Union::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Union {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Union {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
