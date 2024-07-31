use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Release;
impl Release {
    pub const fn as_str() -> &'static str {
        "RELEASE"
    }
}

impl PartialEq<&str> for Release {
    fn eq(&self, other: &&str) -> bool {
        Release::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Release> for &str {
    fn eq(&self, _: &Release) -> bool {
        Release::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Release {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Release {}
