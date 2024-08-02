mod delete;
mod explain;
mod expression;
mod helpers;
mod insert;
mod keywords;
mod literal_value;
mod router;
mod select;
#[cfg(test)]
mod tests;
mod traits;
mod update;

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
        dbg!(&maybe_keyword);

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
