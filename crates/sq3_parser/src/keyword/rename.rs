use std::{any::Any, fmt::Display};

use crate::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Rename;
impl Rename {
    pub const fn as_str() -> &'static str {
        "RENAME"
    }
}

impl PartialEq<&str> for Rename {
    fn eq(&self, other: &&str) -> bool {
        Rename::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Rename> for &str {
    fn eq(&self, _: &Rename) -> bool {
        Rename::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Rename {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Rename {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
