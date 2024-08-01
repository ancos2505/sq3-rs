use std::{any::Any, fmt::Display};

use crate::query::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct End;
impl End {
    pub const fn as_str() -> &'static str {
        "END"
    }
}

impl PartialEq<&str> for End {
    fn eq(&self, other: &&str) -> bool {
        End::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<End> for &str {
    fn eq(&self, _: &End) -> bool {
        End::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for End {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for End {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
