use std::{any::Any, fmt::Display};

use crate::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Commit;
impl Commit {
    pub const fn as_str() -> &'static str {
        "COMMIT"
    }
}

impl PartialEq<&str> for Commit {
    fn eq(&self, other: &&str) -> bool {
        Commit::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Commit> for &str {
    fn eq(&self, _: &Commit) -> bool {
        Commit::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Commit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Commit {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
