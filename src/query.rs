// mod select;
mod expression;
mod keywords;
mod traits;

use std::{
    str::FromStr,
    time::{Duration, Instant},
};

use traits::{SqliteExpression, SqliteKeyword};

use crate::result::{SqliteError, SqliteResult};

// use self::select::SelectQuery;

#[derive(Debug)]
pub(super) struct SqliteQuery {
    start: Instant,
}

impl SqliteQuery {
    pub fn run(query_str: &str) -> SqliteResult<SqliteQueryOutcome> {
        let timer = Self::timer_start();
        let elapsed = timer.elapsed().as_millis();
        println!("Query elapsed: {elapsed} ms");
        todo!()
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

#[derive(Debug, Default)]
pub struct SqliteRecord(String);

#[derive(Debug)]
pub struct TokenizedSqliteQuery<Q: SqliteKeyword>(Q);

// #[derive(Debug)]
// pub enum TokenizedSqliteQuery {
//     _Todo,
//     // Select(SelectQuery<'a>),
//     // Insert(InsertQuery<'a>),
//     // Update(UpdateQuery<'a>),
//     // Delete(DeleteQuery<'a>),
//     // Replace(ReplaceQuery<'a>),
//     // Create(CreateQuery<'a>),
//     // Alter(AlterQuery<'a>),
//     // Drop(DropQuery<'a>),
//     // Reindex(ReindexQuery<'a>),
//     // Begin(BeginQuery<'a>),
//     // Commit(CommitQuery<'a>),
//     // Rollback(RollbackQuery<'a>),
//     // Analyze(AnalyzeQuery<'a>),
//     // Attach(AttachQuery<'a>),
//     // Detach(DetachQuery<'a>),
//     // Explain(ExplainQuery<'a>),
//     // Pragma(PragmaQuery<'a>),
//     // Vacuum(VacuumQuery<'a>),
//     // With(WithQuery<'a>),
// }

#[derive(Debug)]
pub enum SqliteQueryOutcome {
    Success,
    Failure(SqliteDatabaseError),
}

#[derive(Debug)]
pub enum SqliteDatabaseError {
    _Todo,
}
