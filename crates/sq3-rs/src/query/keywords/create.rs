use std::{any::Any, fmt::Display};

use crate::query::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Create;
impl Create {
    pub const fn as_str() -> &'static str {
        "CREATE"
    }
}

impl PartialEq<&str> for Create {
    fn eq(&self, other: &&str) -> bool {
        Create::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Create> for &str {
    fn eq(&self, _: &Create) -> bool {
        Create::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Create {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Create {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
