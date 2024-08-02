use std::{any::Any, fmt::Display};

use crate::query::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Over;
impl Over {
    pub const fn as_str() -> &'static str {
        "OVER"
    }
}

impl PartialEq<&str> for Over {
    fn eq(&self, other: &&str) -> bool {
        Over::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Over> for &str {
    fn eq(&self, _: &Over) -> bool {
        Over::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Over {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Over {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
