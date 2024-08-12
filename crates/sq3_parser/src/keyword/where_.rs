use std::{any::Any, fmt::Display};

use crate::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Where;
impl Where {
    pub const fn as_str() -> &'static str {
        "WHERE"
    }
    pub const fn len() -> usize {
        5
    }
}

impl PartialEq<&str> for Where {
    fn eq(&self, other: &&str) -> bool {
        Where::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Where> for &str {
    fn eq(&self, _: &Where) -> bool {
        Where::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Where {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Where {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
