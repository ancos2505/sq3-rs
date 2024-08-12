use std::{any::Any, fmt::Display};

use crate::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Fail;
impl Fail {
    pub const fn as_str() -> &'static str {
        "FAIL"
    }
}

impl PartialEq<&str> for Fail {
    fn eq(&self, other: &&str) -> bool {
        Fail::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Fail> for &str {
    fn eq(&self, _: &Fail) -> bool {
        Fail::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Fail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Fail {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
