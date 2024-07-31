use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Match;
impl Match {
    pub const fn as_str() -> &'static str {
        "MATCH"
    }
}

impl PartialEq<&str> for Match {
    fn eq(&self, other: &&str) -> bool {
        Match::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Match> for &str {
    fn eq(&self, _: &Match) -> bool {
        Match::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Match {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Match {}
