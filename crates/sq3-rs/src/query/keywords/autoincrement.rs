use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Autoincrement;
impl Autoincrement {
    pub const fn as_str() -> &'static str {
        "AUTOINCREMENT"
    }
}

impl PartialEq<&str> for Autoincrement {
    fn eq(&self, other: &&str) -> bool {
        Autoincrement::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Autoincrement> for &str {
    fn eq(&self, _: &Autoincrement) -> bool {
        Autoincrement::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Autoincrement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Autoincrement {}
