use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Cast;
impl Cast {
    pub const fn as_str() -> &'static str {
        "CAST"
    }
}

impl PartialEq<&str> for Cast {
    fn eq(&self, other: &&str) -> bool {
        Cast::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Cast> for &str {
    fn eq(&self, _: &Cast) -> bool {
        Cast::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Cast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Cast {}
