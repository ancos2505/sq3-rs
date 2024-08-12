use std::{any::Any, fmt::Display};

use crate::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct WhiteSpace;
impl WhiteSpace {
    pub const fn as_str() -> &'static str {
        " "
    }
    pub const fn len() -> usize {
        1
    }
}

impl PartialEq<&str> for WhiteSpace {
    fn eq(&self, other: &&str) -> bool {
        WhiteSpace::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<WhiteSpace> for &str {
    fn eq(&self, _: &WhiteSpace) -> bool {
        WhiteSpace::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for WhiteSpace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for WhiteSpace {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
