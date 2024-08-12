use crate::{
    explain::ExplainStmt,
    helpers::ASCII_WHITESPACE_CHAR,
    keyword::{
        Keyword, KeywordDelete, KeywordExplain, KeywordInsert, KeywordSelect, KeywordUpdate,
        KeywordWith,
    },
    result::{ParserResult, Sq3ParserError},
    stmt::{DeleteStmt, InsertStmt, SelectStmt, UpdateStmt},
};

use super::{traits::SqliteStatement, SqliteQueryOutcome};

#[derive(Debug)]
pub(super) struct QueryRouter;

impl QueryRouter {
    pub(super) fn run(sql: &str) -> ParserResult<SqliteQueryOutcome> {
        let (current_to_parse, next_to_parse) = sql
            .trim()
            .split_once(ASCII_WHITESPACE_CHAR)
            .ok_or(Sq3ParserError(
                "Invalid query. Reason: There is no whitespaces.".into(),
            ))?;

        let keyword = current_to_parse.parse::<Keyword>()?;

        if keyword.get().downcast_ref::<KeywordExplain>().is_some() {
            return Err(Sq3ParserError(format!(
                "{} is not supported.",
                KeywordExplain
            )));
        }

        if keyword.get().downcast_ref::<KeywordWith>().is_some() {
            return Err(Sq3ParserError(format!("{} is not supported.", KeywordWith)));
        }

        if keyword.get().downcast_ref::<KeywordSelect>().is_some() {
            // TODO
            // SelectStmt::run(next_to_parse)
            InsertStmt::run(next_to_parse)
        } else if keyword.get().downcast_ref::<KeywordInsert>().is_some() {
            InsertStmt::run(next_to_parse)
        } else if keyword.get().downcast_ref::<KeywordUpdate>().is_some() {
            UpdateStmt::run(next_to_parse)
        } else if keyword.get().downcast_ref::<KeywordDelete>().is_some() {
            DeleteStmt::run(next_to_parse)
        } else if keyword.get().downcast_ref::<KeywordExplain>().is_some() {
            ExplainStmt::run(next_to_parse)
        } else {
            // TODO
            Err(Sq3ParserError(format!(
                "Can't found valid query for {:?}",
                current_to_parse
            )))
        }
    }
}
