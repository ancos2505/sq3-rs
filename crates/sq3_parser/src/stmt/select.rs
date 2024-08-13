//! # SELECT
//!
//!  The SELECT statement is used to query the database. The result of a SELECT
//! is zero or more rows of data where each row has a fixed number of columns. A
//! SELECT statement does not make any changes to the database.
//!
//!  The core of a SELECT statement is a "simple SELECT" shown by the
//! select-core and simple-select-stmt syntax diagrams below. In practice, most
//! SELECT statements are simple SELECT statements.
//!
//! **References:** - https://www.sqlite.org/lang_select.html#simpleselect
//!                 - https://www.sqlite.org/syntaxdiagrams.html#select-core
//!                 - https://www.sqlite.org/syntax/select-core.html
//!                 - https://www.sqlite.org/syntaxdiagrams.html#select-stmt
//!                 - https://www.sqlite.org/syntax/simple-select-stmt.html
//!

mod parser;

use crate::{
    expression::SqliteExpression,
    keyword::{KeywordFrom, KeywordWhere},
    result::ParserResult,
    traits::DistinctProcessing,
    Sq3ParserError,
};

use self::parser::ResultColumns;

pub(crate) use self::parser::SelectParser;

#[derive(Debug, Default)]
pub(crate) struct SelectStmt<'a> {
    pub(crate) input: &'a str,
    pub(crate) distinct: Option<Box<dyn DistinctProcessing>>,
    pub(crate) result_columns: Option<ResultColumns<'a>>,
    pub(crate) from: Option<KeywordFrom>,
    pub(crate) origin: Option<TableName<'a>>,
    // TODO
    pub(crate) r#where: Option<KeywordWhere>,
    // TODO
    pub(crate) r#where_expr: Option<SqliteExpression>,
    // TODO
    pub(crate) expr: Option<SqliteExpression>,
}

#[derive(Debug, PartialEq, Eq)]
struct TableName<'a>(&'a str);
impl<'a> TableName<'a> {
    pub fn parse(s: &'a str) -> ParserResult<Self> {
        let error = Sq3ParserError("Invalid TableName".into());
        let input = s.trim();

        for (idx, c) in input.chars().enumerate() {
            if (idx == 0) && !c.is_ascii() {
                return Err(error);
            }
            if c.is_ascii_digit() || c.is_ascii_lowercase() || c == '_' {
                continue;
            } else {
                return Err(error);
            }
        }
        return Ok(Self(input));
    }
}
