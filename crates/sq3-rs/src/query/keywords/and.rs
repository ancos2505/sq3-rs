use std::{any::Any, fmt::Display};

use crate::query::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct And;
impl And {
    pub const fn as_str() -> &'static str {
        "AND"
    }
}

impl PartialEq<&str> for And {
    fn eq(&self, other: &&str) -> bool {
        And::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<And> for &str {
    fn eq(&self, _: &And) -> bool {
        And::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for And {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for And {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
