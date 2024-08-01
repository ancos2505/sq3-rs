use crate::{
    query::{
        delete::DeleteStmt,
        insert::InsertStmt,
        keywords::{Delete, Insert, Keyword, Select, Update},
        select::SelectStmt,
        update::UpdateStmt,
    },
    result::{SqlParserError, SqliteError},
    SqliteResult,
};

use super::{traits::SqliteStatement, SqliteQueryOutcome};

#[derive(Debug)]
pub(super) struct QueryRouter;

impl QueryRouter {
    pub(super) fn run(sql: &str) -> SqliteResult<SqliteQueryOutcome> {
        let mut iter = sql.split_ascii_whitespace();
        let maybe_keyword = iter.next();
        dbg!(&maybe_keyword);
        let sql_statement_content = iter.next().unwrap_or_default();

        let keyword = maybe_keyword.unwrap().parse::<Keyword>()?;

        if keyword.get().downcast_ref::<Select>().is_some() {
            SelectStmt::run(sql_statement_content)
        } else if keyword.get().downcast_ref::<Insert>().is_some() {
            InsertStmt::run(sql_statement_content)
        } else if keyword.get().downcast_ref::<Update>().is_some() {
            UpdateStmt::run(sql_statement_content)
        } else if keyword.get().downcast_ref::<Delete>().is_some() {
            DeleteStmt::run(sql_statement_content)
        } else {
            // TODO
            Err(SqliteError::SqlParser(SqlParserError(format!(
                "Can't found valid query for {:?}",
                maybe_keyword
            ))))
        }
    }
}
