use std::{any::Any, fmt::Display};

use crate::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Current_date;
impl Current_date {
    pub const fn as_str() -> &'static str {
        "CURRENT_DATE"
    }
}

impl PartialEq<&str> for Current_date {
    fn eq(&self, other: &&str) -> bool {
        Current_date::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Current_date> for &str {
    fn eq(&self, _: &Current_date) -> bool {
        Current_date::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Current_date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Current_date {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
