use crate::result::ParserResult;

use super::{traits::SqliteStatement, SqliteDatabaseError, SqliteQueryOutcome};

#[derive(Debug, Default)]
pub(super) struct ExplainStmt<'a> {
    input: &'a str, // TODO
}

impl<'a> SqliteStatement<'a> for ExplainStmt<'a> {
    fn run(stmt_content: &str) -> ParserResult<SqliteQueryOutcome> {
        Ok(SqliteQueryOutcome::Failure(SqliteDatabaseError::_Todo))
    }
}
