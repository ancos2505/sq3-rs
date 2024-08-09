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

// #[cfg(test)]
// mod tests;

use crate::{
    query::{
        expression::SqliteExpression,
        keyword::{Keyword, KeywordAll, KeywordDistinct, KeywordFrom, KeywordWhere},
        SqliteDatabaseError,
    },
    result::{SqlParserError, SqliteError, SqliteResult},
};

use crate::query::{
    traits::{DistinctProcessing, SqliteStatement},
    SqliteQueryOutcome,
};

pub(super) use self::parser::{Initial, SelectParser};

#[derive(Debug, Default)]
pub(crate) struct SelectStmt<'a> {
    pub(self) distinct: Option<Box<dyn DistinctProcessing>>,
    pub(self) result_columns: Option<ResultColumns<'a>>,
    pub(self) from: Option<KeywordFrom>,
    // TODO
    pub(self) origin: Option<TableName<'a>>,
    // TODO
    pub(self) r#where: Option<KeywordWhere>,

    pub(self) r#where_expr: Option<SqliteExpression>,

    pub(self) expr: Option<SqliteExpression>,
}

// impl<'a> SelectStmt<'a> {
//     fn parse(sql: &'a str) -> SqliteResult<Self>
//     where
//         Self: 'a,
//     {
//         let cleaned_sql =
//             sql.trim()
//                 .split(';')
//                 .next()
//                 .ok_or(SqliteError::SqlParser(SqlParserError(
//                     "Invalid SQL. Can't start a query with `;`".into(),
//                 )))?;

//         // TODO Implement chunk navigator
//         let mut iter = cleaned_sql.split_whitespace();

//         let mut stmt = Self::default();

//         // TODO Implement chunk navigator
//         while let Some(chunk) = iter.next() {
//             dbg!(1, &stmt);
//             if stmt.distinct.is_none() {
//                 if let Some(keyword) = chunk.parse::<Keyword>().ok() {
//                     if keyword.get().downcast_ref::<KeywordDistinct>().is_some() {
//                         let distinct = keyword.into_inner().downcast::<KeywordDistinct>().ok();

//                         stmt.distinct = distinct.map(|inner| inner as Box<dyn DistinctProcessing>);
//                         continue;
//                     }
//                 }
//             }
//             if stmt.distinct.is_none() {
//                 stmt.distinct = Some(Box::new(KeywordAll) as Box<dyn DistinctProcessing>);
//             }

//             dbg!(2, chunk, &stmt);
//             if stmt.distinct.is_some() && stmt.result_columns.is_none() {
//                 if let Some(result_columns) = ResultColumns::parse(chunk).ok() {
//                     stmt.result_columns = Some(result_columns);
//                     continue;
//                 }
//             }

//             dbg!(3, chunk, &stmt);
//             if stmt.distinct.is_some() && stmt.result_columns.is_some() && stmt.from.is_none() {
//                 if let Some(keyword) = chunk.parse::<Keyword>().ok() {
//                     if keyword.get().downcast_ref::<KeywordFrom>().is_some() {
//                         let from = keyword
//                             .into_inner()
//                             .downcast::<KeywordFrom>()
//                             .ok()
//                             .map(|inner| *inner);

//                         stmt.from = from;
//                         continue;
//                     }
//                 }
//             }

//             dbg!(4, chunk, &stmt);
//             if stmt.distinct.is_some()
//                 && stmt.result_columns.is_some()
//                 && stmt.from.is_some()
//                 && stmt.origin.is_none()
//             {
//                 if let Some(table_name) = TableName::parse(chunk).ok() {
//                     stmt.origin = Some(table_name);
//                     continue;
//                 }
//             }

//             dbg!(5, chunk, &stmt);
//             if stmt.distinct.is_some()
//                 && stmt.result_columns.is_some()
//                 && stmt.from.is_some()
//                 && stmt.origin.is_some()
//                 && stmt.r#where.is_none()
//             {
//                 if let Some(keyword) = chunk.parse::<Keyword>().ok() {
//                     if keyword.get().downcast_ref::<KeywordWhere>().is_some() {
//                         let keyword_where = keyword
//                             .into_inner()
//                             .downcast::<KeywordWhere>()
//                             .ok()
//                             .map(|k| *k);

//                         stmt.r#where = keyword_where;
//                         continue;
//                     }
//                 }
//             }

//             dbg!(6, chunk, &stmt);
//             if stmt.distinct.is_some()
//                 && stmt.result_columns.is_some()
//                 && stmt.from.is_some()
//                 && stmt.origin.is_some()
//                 && stmt.r#where.is_none()
//             {
//                 if let Some(keyword) = chunk.parse::<Keyword>().ok() {
//                     if keyword.get().downcast_ref::<KeywordWhere>().is_some() {
//                         let keyword_where = keyword
//                             .into_inner()
//                             .downcast::<KeywordWhere>()
//                             .ok()
//                             .map(|k| *k);

//                         stmt.r#where = keyword_where;
//                         continue;
//                     }
//                 }
//             }

//             if stmt.distinct.is_some()
//                 && stmt.result_columns.is_some()
//                 && stmt.from.is_some()
//                 && stmt.origin.is_some()
//                 && stmt.r#where.is_some()
//                 && stmt.where_expr.is_none()
//             {
//                 stmt.where_expr = Some(SqliteExpression::from(chunk.to_string()));
//             }

//             // Expr
//             if stmt.distinct.is_some()
//                 && stmt.result_columns.is_none()
//                 && stmt.from.is_none()
//                 && stmt.origin.is_none()
//                 && stmt.r#where.is_none()
//                 && stmt.where_expr.is_none()
//             {
//                 stmt.expr = Some(SqliteExpression::from(chunk.to_string()));
//             }
//         }
//         Ok(stmt)
//     }
// }

// impl<'a> SqliteStatement<'a> for SelectStmt<'a> {
//     fn run(sql: &'a str) -> SqliteResult<SqliteQueryOutcome> {
//         let stmt: SelectStmt<'a> = Self::parse(sql)?;
//         println!("Parsed: {stmt:?}");
//         Ok(SqliteQueryOutcome::Failure(SqliteDatabaseError::_Todo))
//     }
// }

// TODO
#[derive(Debug, PartialEq, Eq)]
enum ResultColumns<'a> {
    Asterisk,
    Filter(Vec<ColumnName<'a>>),
}
// impl<'a> ResultColumns<'a> {
//     pub fn parse(s: &'a str) -> SqliteResult<Self> {
//         if s.len() == 0 {
//             return Err(SqliteError::SqlParser(SqlParserError(
//                 "ResultColumns not found.".into(),
//             )));
//         }
//         match s.trim() {
//             "*" => Ok(Self::Asterisk),
//             cols_str => {
//                 let mut columns_name = vec![];
//                 let mut iter = cols_str.split(',');
//                 while let Some(col_name) = iter.next().map(|s| s.trim()) {
//                     columns_name.push(ColumnName::parse(col_name)?);
//                 }
//                 Ok(Self::Filter(columns_name))
//             }
//         }
//     }
// }

#[derive(Debug, PartialEq, Eq)]
struct ColumnName<'a>(&'a str);
// impl<'a> ColumnName<'a> {
//     pub fn parse(s: &'a str) -> SqliteResult<Self> {
//         let error = Err(SqliteError::SqlParser(SqlParserError(
//             "Invalid ColumnName".into(),
//         )));
//         for (idx, c) in s.trim().chars().enumerate() {
//             if (idx == 0) && !c.is_ascii_lowercase() {
//                 return error;
//             }
//             if c.is_ascii_digit() || c.is_ascii_lowercase() || c == '_' {
//                 return Ok(Self(s));
//             }
//         }
//         error
//     }
// }

#[derive(Debug, PartialEq, Eq)]
struct TableName<'a>(&'a str);
// impl<'a> TableName<'a> {
//     pub fn parse(s: &'a str) -> SqliteResult<Self> {
//         let error = Err(SqliteError::SqlParser(SqlParserError(
//             "Invalid TableName".into(),
//         )));
//         for (idx, c) in s.trim().chars().enumerate() {
//             if (idx == 0) && !c.is_ascii_lowercase() {
//                 return error;
//             }
//             if c.is_ascii_digit() || c.is_ascii_lowercase() || c == '_' {
//                 return Ok(Self(s));
//             }
//         }
//         error
//     }
// }
