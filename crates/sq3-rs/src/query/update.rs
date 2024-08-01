use crate::result::SqliteResult;

use super::{traits::SqliteStatement, SqliteDatabaseError, SqliteQueryOutcome};

#[derive(Debug, Default)]
pub(super) struct UpdateStmt<'a> {
    input: &'a str, // TODO
}

impl<'a> SqliteStatement for UpdateStmt<'a> {
    fn run(stmt_content: &str) -> SqliteResult<SqliteQueryOutcome> {
        Ok(SqliteQueryOutcome::Failure(SqliteDatabaseError::_Todo))
    }
}
