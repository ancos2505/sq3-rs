use std::{any::Any, fmt::Display};

use crate::query::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Current_time;
impl Current_time {
    pub const fn as_str() -> &'static str {
        "CURRENT_TIME"
    }
}

impl PartialEq<&str> for Current_time {
    fn eq(&self, other: &&str) -> bool {
        Current_time::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Current_time> for &str {
    fn eq(&self, _: &Current_time) -> bool {
        Current_time::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Current_time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Current_time {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
