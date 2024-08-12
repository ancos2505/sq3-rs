use std::{any::Any, fmt::Display};

use crate::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Desc;
impl Desc {
    pub const fn as_str() -> &'static str {
        "DESC"
    }
}

impl PartialEq<&str> for Desc {
    fn eq(&self, other: &&str) -> bool {
        Desc::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Desc> for &str {
    fn eq(&self, _: &Desc) -> bool {
        Desc::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Desc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Desc {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
