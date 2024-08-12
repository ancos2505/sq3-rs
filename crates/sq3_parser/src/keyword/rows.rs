use std::{any::Any, fmt::Display};

use crate::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Rows;
impl Rows {
    pub const fn as_str() -> &'static str {
        "ROWS"
    }
}

impl PartialEq<&str> for Rows {
    fn eq(&self, other: &&str) -> bool {
        Rows::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Rows> for &str {
    fn eq(&self, _: &Rows) -> bool {
        Rows::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Rows {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Rows {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
