use std::{any::Any, fmt::Display};

use crate::query::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Action;
impl Action {
    pub const fn as_str() -> &'static str {
        "ACTION"
    }
}

impl PartialEq<&str> for Action {
    fn eq(&self, other: &&str) -> bool {
        Action::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Action> for &str {
    fn eq(&self, _: &Action) -> bool {
        Action::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Action {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
