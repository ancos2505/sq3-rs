use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Rollback;
impl Rollback {
    pub const fn as_str() -> &'static str {
        "ROLLBACK"
    }
}

impl PartialEq<&str> for Rollback {
    fn eq(&self, other: &&str) -> bool {
        Rollback::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Rollback> for &str {
    fn eq(&self, _: &Rollback) -> bool {
        Rollback::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Rollback {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Rollback {}
