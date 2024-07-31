use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Alter;
impl Alter {
    pub const fn as_str() -> &'static str {
        "ALTER"
    }
}

impl PartialEq<&str> for Alter {
    fn eq(&self, other: &&str) -> bool {
        Alter::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Alter> for &str {
    fn eq(&self, _: &Alter) -> bool {
        Alter::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Alter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Alter {}
