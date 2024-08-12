use crate::result::ParserResult;

use crate::{traits::SqliteStatement, SqliteDatabaseError, SqliteQueryOutcome};

#[derive(Debug, Default)]
pub(crate) struct InsertStmt<'a> {
    input: &'a str, // TODO
}

impl<'a> SqliteStatement<'a> for InsertStmt<'a> {
    fn run(stmt_content: &str) -> ParserResult<SqliteQueryOutcome> {
        Ok(SqliteQueryOutcome::Failure(SqliteDatabaseError::_Todo))
    }
}
