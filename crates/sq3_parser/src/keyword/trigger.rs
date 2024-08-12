use std::{any::Any, fmt::Display};

use crate::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Trigger;
impl Trigger {
    pub const fn as_str() -> &'static str {
        "TRIGGER"
    }
}

impl PartialEq<&str> for Trigger {
    fn eq(&self, other: &&str) -> bool {
        Trigger::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Trigger> for &str {
    fn eq(&self, _: &Trigger) -> bool {
        Trigger::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Trigger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Trigger {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
