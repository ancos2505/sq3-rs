use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Order;
impl Order {
    pub const fn as_str() -> &'static str {
        "ORDER"
    }
}

impl PartialEq<&str> for Order {
    fn eq(&self, other: &&str) -> bool {
        Order::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Order> for &str {
    fn eq(&self, _: &Order) -> bool {
        Order::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Order {}
