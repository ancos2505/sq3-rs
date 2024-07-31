use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Virtual;
impl Virtual {
    pub const fn as_str() -> &'static str {
        "VIRTUAL"
    }
}

impl PartialEq<&str> for Virtual {
    fn eq(&self, other: &&str) -> bool {
        Virtual::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Virtual> for &str {
    fn eq(&self, _: &Virtual) -> bool {
        Virtual::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Virtual {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Virtual {}
