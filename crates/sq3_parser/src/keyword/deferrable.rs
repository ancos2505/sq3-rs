use std::{any::Any, fmt::Display};

use crate::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Deferrable;
impl Deferrable {
    pub const fn as_str() -> &'static str {
        "DEFERRABLE"
    }
}

impl PartialEq<&str> for Deferrable {
    fn eq(&self, other: &&str) -> bool {
        Deferrable::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Deferrable> for &str {
    fn eq(&self, _: &Deferrable) -> bool {
        Deferrable::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Deferrable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Deferrable {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
