use std::{any::Any, fmt::Display};

use crate::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Analyze;
impl Analyze {
    pub const fn as_str() -> &'static str {
        "ANALYZE"
    }
}

impl PartialEq<&str> for Analyze {
    fn eq(&self, other: &&str) -> bool {
        Analyze::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Analyze> for &str {
    fn eq(&self, _: &Analyze) -> bool {
        Analyze::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Analyze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Analyze {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
