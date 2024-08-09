mod explain;
mod expression;
mod helpers;
mod keyword;
mod literal_value;
mod router;
mod stmt;
mod traits;
mod white_space;

#[cfg(test)]
mod tests;

use crate::result::{SqlParserError, SqliteError, SqliteResult};

use self::router::QueryRouter;

pub(self) use self::white_space::WhiteSpace;

pub use self::helpers::{SqliteDatabaseError, SqliteQueryOutcome, SqliteRecord};

/// ## SqliteQuery
///
/// **Reference:** https://www.sqlite.org/syntaxdiagrams.html#sql-stmt
///
#[derive(Debug)]
pub(super) struct SqliteQuery;

impl SqliteQuery {
    pub fn run(sql: &str) -> SqliteResult<SqliteQueryOutcome> {
        if !sql.ends_with(";") {
            return Err(SqliteError::SqlParser(SqlParserError(
                "Invalid query. Reason: Every query must ends with `;`.".into(),
            )));
        }

        let db_outcome = QueryRouter::run(sql)?;

        Ok(db_outcome)
    }
}
