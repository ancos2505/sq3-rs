use crate::{
    query::{
        explain::ExplainStmt,
        keyword::{Delete, Explain, Insert, Keyword, Select, Update, With},
        stmt::{DeleteStmt, InsertStmt, SelectStmt, UpdateStmt},
    },
    result::{SqlParserError, SqliteError},
    SqliteResult,
};

use super::{traits::SqliteStatement, SqliteQueryOutcome};

#[derive(Debug)]
pub(super) struct QueryRouter;

impl QueryRouter {
    pub(super) fn run(sql: &str) -> SqliteResult<SqliteQueryOutcome> {
        let (sql_stmt, sql_to_parse) =
            sql.trim()
                .split_once(' ')
                .ok_or(SqliteError::SqlParser(SqlParserError(
                    "Invalid query. Reason: There is no whitespaces.".into(),
                )))?;
        dbg!(sql_stmt, sql_to_parse);

        let keyword = sql_stmt.parse::<Keyword>()?;

        if keyword.get().downcast_ref::<Explain>().is_some() {
            return Err(SqliteError::SqlParser(SqlParserError(format!(
                "{} is not supported.",
                Explain::as_str()
            ))));
        }

        if keyword.get().downcast_ref::<With>().is_some() {
            return Err(SqliteError::SqlParser(SqlParserError(format!(
                "{} is not supported.",
                With::as_str()
            ))));
        }

        if keyword.get().downcast_ref::<Select>().is_some() {
            SelectStmt::run(sql_to_parse)
        } else if keyword.get().downcast_ref::<Insert>().is_some() {
            InsertStmt::run(sql_to_parse)
        } else if keyword.get().downcast_ref::<Update>().is_some() {
            UpdateStmt::run(sql_to_parse)
        } else if keyword.get().downcast_ref::<Delete>().is_some() {
            DeleteStmt::run(sql_to_parse)
        } else if keyword.get().downcast_ref::<Explain>().is_some() {
            ExplainStmt::run(sql_to_parse)
        } else {
            // TODO
            Err(SqliteError::SqlParser(SqlParserError(format!(
                "Can't found valid query for {:?}",
                sql_stmt
            ))))
        }
    }
}
