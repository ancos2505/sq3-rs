use std::{any::Any, fmt::Display};

use crate::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Raise;
impl Raise {
    pub const fn as_str() -> &'static str {
        "RAISE"
    }
}

impl PartialEq<&str> for Raise {
    fn eq(&self, other: &&str) -> bool {
        Raise::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Raise> for &str {
    fn eq(&self, _: &Raise) -> bool {
        Raise::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Raise {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Raise {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
