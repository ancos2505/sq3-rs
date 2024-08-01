use std::{any::Any, fmt::Display};

use crate::query::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Initially;
impl Initially {
    pub const fn as_str() -> &'static str {
        "INITIALLY"
    }
}

impl PartialEq<&str> for Initially {
    fn eq(&self, other: &&str) -> bool {
        Initially::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Initially> for &str {
    fn eq(&self, _: &Initially) -> bool {
        Initially::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Initially {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Initially {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
