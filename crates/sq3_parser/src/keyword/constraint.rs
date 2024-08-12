use std::{any::Any, fmt::Display};

use crate::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Constraint;
impl Constraint {
    pub const fn as_str() -> &'static str {
        "CONSTRAINT"
    }
}

impl PartialEq<&str> for Constraint {
    fn eq(&self, other: &&str) -> bool {
        Constraint::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Constraint> for &str {
    fn eq(&self, _: &Constraint) -> bool {
        Constraint::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Constraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Constraint {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
