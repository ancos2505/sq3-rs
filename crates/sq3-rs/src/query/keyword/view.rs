use std::{any::Any, fmt::Display};

use crate::query::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct View;
impl View {
    pub const fn as_str() -> &'static str {
        "VIEW"
    }
}

impl PartialEq<&str> for View {
    fn eq(&self, other: &&str) -> bool {
        View::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<View> for &str {
    fn eq(&self, _: &View) -> bool {
        View::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for View {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for View {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
