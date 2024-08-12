use std::{any::Any, fmt::Display};

use crate::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct If;
impl If {
    pub const fn as_str() -> &'static str {
        "IF"
    }
}

impl PartialEq<&str> for If {
    fn eq(&self, other: &&str) -> bool {
        If::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<If> for &str {
    fn eq(&self, _: &If) -> bool {
        If::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for If {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for If {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
