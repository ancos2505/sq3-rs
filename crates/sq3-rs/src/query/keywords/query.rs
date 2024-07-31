use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Query;
impl Query {
    pub const fn as_str() -> &'static str {
        "QUERY"
    }
}

impl PartialEq<&str> for Query {
    fn eq(&self, other: &&str) -> bool {
        Query::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Query> for &str {
    fn eq(&self, _: &Query) -> bool {
        Query::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Query {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Query {}
