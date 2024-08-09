use std::{any::Any, fmt::Display};

use crate::query::traits::{DistinctProcessing, SqliteKeyword};

impl DistinctProcessing for Distinct {}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Distinct;
impl Distinct {
    pub const fn as_str() -> &'static str {
        "DISTINCT"
    }
    pub const fn len() -> usize {
        8
    }
}

impl PartialEq<&str> for Distinct {
    fn eq(&self, other: &&str) -> bool {
        Distinct::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Distinct> for &str {
    fn eq(&self, _: &Distinct) -> bool {
        Distinct::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Distinct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Distinct {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
