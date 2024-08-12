use std::{any::Any, fmt::Display};

use crate::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Exclude;
impl Exclude {
    pub const fn as_str() -> &'static str {
        "EXCLUDE"
    }
}

impl PartialEq<&str> for Exclude {
    fn eq(&self, other: &&str) -> bool {
        Exclude::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Exclude> for &str {
    fn eq(&self, _: &Exclude) -> bool {
        Exclude::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Exclude {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Exclude {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
