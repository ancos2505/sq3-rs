use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct Transaction;
impl Transaction {
    pub const fn as_str() -> &'static str {
        "TRANSACTION"
    }
}

impl PartialEq<&str> for Transaction {
    fn eq(&self, other: &&str) -> bool {
        Transaction::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Transaction> for &str {
    fn eq(&self, _: &Transaction) -> bool {
        Transaction::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Transaction {}
