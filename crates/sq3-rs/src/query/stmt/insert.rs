use crate::result::SqliteResult;

use crate::query::{traits::SqliteStatement, SqliteDatabaseError, SqliteQueryOutcome};

#[derive(Debug, Default)]
pub(crate) struct InsertStmt<'a> {
    input: &'a str, // TODO
}

impl<'a> SqliteStatement<'a> for InsertStmt<'a> {
    fn run(stmt_content: &str) -> SqliteResult<SqliteQueryOutcome> {
        Ok(SqliteQueryOutcome::Failure(SqliteDatabaseError::_Todo))
    }
}
