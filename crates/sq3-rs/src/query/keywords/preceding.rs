use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Preceding;
impl Preceding {
    pub const fn as_str() -> &'static str {
        "PRECEDING"
    }
}

impl PartialEq<&str> for Preceding {
    fn eq(&self, other: &&str) -> bool {
        Preceding::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Preceding> for &str {
    fn eq(&self, _: &Preceding) -> bool {
        Preceding::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Preceding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Preceding {}
