use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Plan;
impl Plan {
    pub const fn as_str() -> &'static str {
        "PLAN"
    }
}

impl PartialEq<&str> for Plan {
    fn eq(&self, other: &&str) -> bool {
        Plan::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Plan> for &str {
    fn eq(&self, _: &Plan) -> bool {
        Plan::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Plan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Plan {}
