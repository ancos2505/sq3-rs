use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Vacuum;
impl Vacuum {
    pub const fn as_str() -> &'static str {
        "VACUUM"
    }
}

impl PartialEq<&str> for Vacuum {
    fn eq(&self, other: &&str) -> bool {
        Vacuum::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Vacuum> for &str {
    fn eq(&self, _: &Vacuum) -> bool {
        Vacuum::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Vacuum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Vacuum {}
