use std::{str::FromStr, time::Instant};

use crate::result::SqliteError;

#[derive(Debug)]
pub struct SqliteQuery {
    query_str: String,
    query_token: TokenizedSqliteQuery,
    start: Instant,
    end: Option<Instant>,
    outcome: Option<SqliteQueryOutcome>,
}

impl SqliteQuery {
    pub fn set_outcome(&mut self, outcome: SqliteQueryOutcome) {
        self.outcome = Some(outcome);
        self.end = Some(Instant::now())
    }
}

impl FromStr for SqliteQuery {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let query = Self {
            query_str: s.into(),
            query_token: TokenizedSqliteQuery,
            start: Instant::now(),
            end: None,
            outcome: None,
        };
        Ok(query)
    }
}

#[derive(Debug, Default)]
pub struct SqliteRecord(String);

#[derive(Debug)]
pub struct TokenizedSqliteQuery;

#[derive(Debug)]
pub enum SqliteQueryOutcome {
    Success,
    Failure(SqliteDatabaseError),
}

#[derive(Debug)]
pub enum SqliteDatabaseError {
    _Todo,
}
