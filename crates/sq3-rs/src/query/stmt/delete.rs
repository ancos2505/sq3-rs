use crate::result::SqliteResult;

use crate::query::{traits::SqliteStatement, SqliteDatabaseError, SqliteQueryOutcome};

#[derive(Debug, Default)]
pub(crate) struct DeleteStmt<'a> {
    input: &'a str, // TODO
}

impl<'a> SqliteStatement for DeleteStmt<'a> {
    fn run(stmt_content: &str) -> SqliteResult<SqliteQueryOutcome> {
        Ok(SqliteQueryOutcome::Failure(SqliteDatabaseError::_Todo))
    }
}
