use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Filter;
impl Filter {
    pub const fn as_str() -> &'static str {
        "FILTER"
    }
}

impl PartialEq<&str> for Filter {
    fn eq(&self, other: &&str) -> bool {
        Filter::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Filter> for &str {
    fn eq(&self, _: &Filter) -> bool {
        Filter::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Filter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Filter {}
