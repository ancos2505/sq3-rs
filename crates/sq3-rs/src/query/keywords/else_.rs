use std::{any::Any, fmt::Display};

use crate::query::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Else;
impl Else {
    pub const fn as_str() -> &'static str {
        "ELSE"
    }
}

impl PartialEq<&str> for Else {
    fn eq(&self, other: &&str) -> bool {
        Else::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Else> for &str {
    fn eq(&self, _: &Else) -> bool {
        Else::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Else {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Else {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
