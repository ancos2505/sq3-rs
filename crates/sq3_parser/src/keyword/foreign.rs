use std::{any::Any, fmt::Display};

use crate::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Foreign;
impl Foreign {
    pub const fn as_str() -> &'static str {
        "FOREIGN"
    }
}

impl PartialEq<&str> for Foreign {
    fn eq(&self, other: &&str) -> bool {
        Foreign::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Foreign> for &str {
    fn eq(&self, _: &Foreign) -> bool {
        Foreign::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Foreign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Foreign {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
