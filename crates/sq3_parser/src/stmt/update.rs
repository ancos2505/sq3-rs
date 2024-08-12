use crate::result::ParserResult;

use crate::{
    helpers::{SqliteDatabaseError, SqliteQueryOutcome},
    traits::SqliteStatement,
};

#[derive(Debug, Default)]
pub(crate) struct UpdateStmt<'a> {
    input: &'a str, // TODO
}

impl<'a> SqliteStatement<'a> for UpdateStmt<'a> {
    fn run(stmt_content: &str) -> ParserResult<SqliteQueryOutcome> {
        Ok(SqliteQueryOutcome::Failure(SqliteDatabaseError::_Todo))
    }
}
