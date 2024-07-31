use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Desc;
impl Desc {
    pub const fn as_str() -> &'static str {
        "DESC"
    }
}

impl PartialEq<&str> for Desc {
    fn eq(&self, other: &&str) -> bool {
        Desc::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Desc> for &str {
    fn eq(&self, _: &Desc) -> bool {
        Desc::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Desc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Desc {}
