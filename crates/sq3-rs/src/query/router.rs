use crate::{
    query::{
        explain::ExplainStmt,
        helpers::ASCII_WHITESPACE_CHAR,
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
        let (current_to_parse, next_to_parse) = sql
            .trim()
            .split_once(ASCII_WHITESPACE_CHAR)
            .ok_or(SqliteError::SqlParser(SqlParserError(
                "Invalid query. Reason: There is no whitespaces.".into(),
            )))?;

        let keyword = current_to_parse.parse::<Keyword>()?;

        if keyword.get().downcast_ref::<Explain>().is_some() {
            return Err(SqliteError::SqlParser(SqlParserError(format!(
                "{} is not supported.",
                Explain
            ))));
        }

        if keyword.get().downcast_ref::<With>().is_some() {
            return Err(SqliteError::SqlParser(SqlParserError(format!(
                "{} is not supported.",
                With
            ))));
        }

        if keyword.get().downcast_ref::<Select>().is_some() {
            SelectStmt::run(next_to_parse)
        } else if keyword.get().downcast_ref::<Insert>().is_some() {
            InsertStmt::run(next_to_parse)
        } else if keyword.get().downcast_ref::<Update>().is_some() {
            UpdateStmt::run(next_to_parse)
        } else if keyword.get().downcast_ref::<Delete>().is_some() {
            DeleteStmt::run(next_to_parse)
        } else if keyword.get().downcast_ref::<Explain>().is_some() {
            ExplainStmt::run(next_to_parse)
        } else {
            // TODO
            Err(SqliteError::SqlParser(SqlParserError(format!(
                "Can't found valid query for {:?}",
                current_to_parse
            ))))
        }
    }
}
