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
//!
//!                 - https://www.sqlite.org/syntaxdiagrams.html#select-stmt
//!                 - https://www.sqlite.org/syntax/simple-select-stmt.html
//!

use std::iter;
use std::str::FromStr;

use super::traits::DistinctProcessing;
use super::{expression::SqliteExpression, traits::SqliteKeyword};
use crate::query::keywords::{All, Distinct, Keyword, KeywordFrom};
use crate::result::{SqlParserError, SqliteError, SqliteResult};

#[derive(Debug, Default)]
pub(super) struct SelectStmt<'a> {
    distinct: Option<Box<dyn DistinctProcessing>>,
    result_columns: Option<ResultColumns<'a>>,
    from: Option<KeywordFrom>,
    // TODO
    origin: Option<TableName<'a>>,
    // distinct: Option<Keyword>,
    // expr: Option<SqliteExpression>,
}

impl<'a> SelectStmt<'a> {
    pub fn run(input: &'a str) -> SqliteResult<Self> {
        let cleaned_sql = input
            .trim()
            .split(';')
            .next()
            .ok_or(SqliteError::SqlParser(SqlParserError(
                "Invalid SQL. Missing `;`".into(),
            )))?;

        let mut iter = cleaned_sql.split_whitespace();

        if let Some(select) = iter.next() {
            if select != "SELECT" {
                return Err(SqliteError::SqlParser(SqlParserError(
                    "Invalid SQL. Missing SELECT keyword".into(),
                )));
            }
        }

        let mut stmt = Self::default();
        // let mut round: usize = 0;
        while let Some(looking_ahead) = iter.next() {
            // round += 1;
            // dbg!(&round);
            // dbg!(&looking_ahead, &stmt);
            // dbg!(&looking_ahead, &stmt);
            if stmt.distinct.is_none() {
                if let Some(keyword) = looking_ahead.parse::<Keyword>().ok() {
                    // dbg!(round, &keyword);
                    if let Some(k) = keyword.0.downcast_ref::<All>() {
                        // dbg!(k);
                        let all = keyword
                            .0
                            .downcast::<All>()
                            .ok()
                            .map(|all| all as Box<dyn DistinctProcessing>);
                        stmt.distinct = all;
                        continue;
                    } else if let Some(k) = keyword.0.downcast_ref::<Distinct>() {
                        // dbg!(k);
                        let distinct = keyword
                            .0
                            .downcast::<Distinct>()
                            .ok()
                            .map(|distinct| distinct as Box<dyn DistinctProcessing>);
                        stmt.distinct = distinct;
                        continue;
                    }
                }
                stmt.distinct = Some(Box::new(All) as Box<dyn DistinctProcessing>);
            }

            // dbg!(round, &looking_ahead, &stmt);
            if stmt.result_columns.is_none() {
                if let Some(result_columns) = ResultColumns::parse(looking_ahead).ok() {
                    stmt.result_columns = Some(result_columns);
                    continue;
                }
            }

            if stmt.from.is_none() {
                // dbg!(&looking_ahead, &stmt);
                let some_keyword = looking_ahead.parse::<Keyword>();
                // dbg!(&some_keyword);
                if let Some(keyword) = some_keyword.ok() {
                    // dbg!(&keyword);
                    if let Some(k) = keyword.0.downcast_ref::<KeywordFrom>() {
                        // dbg!(k);
                        let from = keyword.0.downcast::<KeywordFrom>().ok();
                        stmt.from = from.map(|boxed| *boxed);
                        continue;
                    }
                }
            }

            if stmt.from.is_some() && stmt.origin.is_none() {
                if let Some(table_name) = TableName::parse(looking_ahead).ok() {
                    stmt.origin = Some(table_name);
                    continue;
                }
            }
        }

        Ok(stmt)
    }
}

#[derive(Debug)]
enum ResultColumns<'a> {
    Asterisk,
    Filter(Vec<ColumnName<'a>>),
}
impl<'a> ResultColumns<'a> {
    pub fn parse(s: &'a str) -> SqliteResult<Self> {
        if s.len() == 0 {
            return Err(SqliteError::SqlParser(SqlParserError(
                "ResultColumns not found.".into(),
            )));
        }
        match s.trim() {
            "*" => Ok(Self::Asterisk),
            cols_str => {
                let mut columns_name = vec![];
                let mut iter = cols_str.split(',');
                while let Some(col_name) = iter.next().map(|s| s.trim()) {
                    columns_name.push(ColumnName::parse(col_name)?);
                }
                Ok(Self::Filter(columns_name))
            }
        }
    }
}

#[derive(Debug)]
struct ColumnName<'a>(&'a str);
impl<'a> ColumnName<'a> {
    pub fn parse(s: &'a str) -> SqliteResult<Self> {
        let error = Err(SqliteError::SqlParser(SqlParserError(
            "Invalid ColumnName".into(),
        )));
        for (idx, c) in s.trim().chars().enumerate() {
            if (idx == 0) && !c.is_ascii_lowercase() {
                return error;
            }
            if c.is_ascii_digit() || c.is_ascii_lowercase() || c == '_' {
                return Ok(Self(s));
            }
        }
        error
    }
}

#[derive(Debug)]
struct TableName<'a>(&'a str);
impl<'a> TableName<'a> {
    pub fn parse(s: &'a str) -> SqliteResult<Self> {
        let error = Err(SqliteError::SqlParser(SqlParserError(
            "Invalid TableName".into(),
        )));
        for (idx, c) in s.trim().chars().enumerate() {
            if (idx == 0) && !c.is_ascii_lowercase() {
                return error;
            }
            if c.is_ascii_digit() || c.is_ascii_lowercase() || c == '_' {
                return Ok(Self(s));
            }
        }
        error
    }
}

#[derive(Debug)]
struct SelectExpr {
    keyword: Option<Keyword>,
    // expr: SqliteExpression,
}

// #[derive(Debug)]
// pub(super) struct SelectStmt<D: DistinctProcessing> {
//     distinct_processing: Option<PhantomData<D>>,
//     result_columns: Vec<String>,
//     from: From,
// }
