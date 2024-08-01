use std::{any::Any, fmt::Display};

use crate::query::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Window;
impl Window {
    pub const fn as_str() -> &'static str {
        "WINDOW"
    }
}

impl PartialEq<&str> for Window {
    fn eq(&self, other: &&str) -> bool {
        Window::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Window> for &str {
    fn eq(&self, _: &Window) -> bool {
        Window::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Window {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Window {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
