mod explain;
mod expression;
mod helpers;
mod keyword;
mod literal_value;
mod router;
mod stmt;
mod traits;

#[cfg(test)]
mod tests;

use std::time::{Duration, Instant};

use crate::result::SqliteResult;

use self::router::QueryRouter;

pub use self::helpers::{SqliteDatabaseError, SqliteQueryOutcome, SqliteRecord};

/// ## SqliteQuery
///
/// **Reference:** https://www.sqlite.org/syntaxdiagrams.html#sql-stmt
///
#[derive(Debug)]
pub(super) struct SqliteQuery {
    start: Instant,
}

impl SqliteQuery {
    pub fn run(sql: &str) -> SqliteResult<SqliteQueryOutcome> {
        let timer = Self::timer_start();

        let maybe_keyword = sql.split_ascii_whitespace().next();

        let db_outcome = QueryRouter::run(sql)?;

        let elapsed = timer.elapsed().as_micros();

        Ok(db_outcome)
    }
    fn timer_start() -> Self {
        Self {
            start: Instant::now(),
        }
    }
    fn elapsed(self) -> Duration {
        Instant::now() - self.start
    }
}
