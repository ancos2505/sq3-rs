use std::{any::Any, fmt::Display};

use crate::query::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Attach;
impl Attach {
    pub const fn as_str() -> &'static str {
        "ATTACH"
    }
}

impl PartialEq<&str> for Attach {
    fn eq(&self, other: &&str) -> bool {
        Attach::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Attach> for &str {
    fn eq(&self, _: &Attach) -> bool {
        Attach::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Attach {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Attach {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
