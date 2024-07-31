use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Temp;
impl Temp {
    pub const fn as_str() -> &'static str {
        "TEMP"
    }
}

impl PartialEq<&str> for Temp {
    fn eq(&self, other: &&str) -> bool {
        Temp::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Temp> for &str {
    fn eq(&self, _: &Temp) -> bool {
        Temp::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Temp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Temp {}
