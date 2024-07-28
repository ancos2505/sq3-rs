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

use std::str::FromStr;

use super::traits::DistinctProcessing;
use super::{expression::SqliteExpression, traits::SqliteKeyword};
use crate::query::keywords::{self, All, Distinct, Keyword};
use crate::result::{SqlParserError, SqliteError, SqliteResult};

#[derive(Debug, Default)]
pub(super) struct SelectStmt {
    distinct: Option<Box<dyn DistinctProcessing>>,
    result_columns: Option<ResultColumns>,
    // distinct: Option<Keyword>,
    // expr: Option<SqliteExpression>,
}

impl SelectStmt {
    pub fn run(sql: &str) -> SqliteResult<Self> {
        let mut iter = sql.trim().split_whitespace();
        let preamble = iter.next();
        assert_eq!(Some("SELECT"), preamble);
        let mut stmt = Self::default();
        while let Some(looking_ahead) = iter.next() {
            dbg!(&looking_ahead);
            if stmt.distinct.is_none() {
                if let Some(keyword) = looking_ahead.parse::<Keyword>().ok() {
                    if let Some(_) = keyword.0.downcast_ref::<All>() {
                        let all = keyword
                            .0
                            .downcast::<All>()
                            .ok()
                            .map(|all| all as Box<dyn DistinctProcessing>);
                        stmt.distinct = all;
                    } else if let Some(_) = keyword.0.downcast_ref::<Distinct>() {
                        let distinct = keyword
                            .0
                            .downcast::<Distinct>()
                            .ok()
                            .map(|distinct| distinct as Box<dyn DistinctProcessing>);
                        stmt.distinct = distinct;
                    }
                }
            }
            if let Some(result_columns) = looking_ahead.parse::<ResultColumns>().ok() {
                stmt.result_columns = Some(result_columns);
            }
            // let expr = looking_ahead.parse::<SqliteExpression>().ok();
        }

        Ok(stmt)
    }
}

#[derive(Debug)]
enum ResultColumns {
    All,
    Filter(Vec<ColumnName>),
}
impl FromStr for ResultColumns {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 0 {
            return Err(SqliteError::SqlParser(SqlParserError(
                "ResultColumns not found.".into(),
            )));
        }
        match s.trim() {
            "*" => Ok(Self::All),
            cols_str => {
                let error = Err(SqliteError::SqlParser(SqlParserError(
                    "Invalid ColumnName".into(),
                )));
                let mut columns_name = vec![];
                let mut iter = cols_str.split(',');
                while let Some(col_name) = iter.next() {
                    for (idx, c) in col_name.chars().enumerate() {
                        if (idx == 0) && !c.is_ascii_alphabetic() {
                            return error;
                        }
                        if !c.is_ascii() {
                            return error;
                        }

                        columns_name.push(ColumnName(col_name.into()))
                    }
                }
                Ok(Self::Filter(columns_name))
            }
        }
    }
}

#[derive(Debug)]
struct ColumnName(String);

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
