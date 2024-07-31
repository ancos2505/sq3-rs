use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Current_timestamp;
impl Current_timestamp {
    pub const fn as_str() -> &'static str {
        "CURRENT_TIMESTAMP"
    }
}

impl PartialEq<&str> for Current_timestamp {
    fn eq(&self, other: &&str) -> bool {
        Current_timestamp::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Current_timestamp> for &str {
    fn eq(&self, _: &Current_timestamp) -> bool {
        Current_timestamp::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Current_timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Current_timestamp {}
