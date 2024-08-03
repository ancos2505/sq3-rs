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

#[cfg(test)]
mod tests;

use crate::{
    query::{
        expression::SqliteExpression,
        keyword::{All, Distinct, Keyword, KeywordFrom},
        SqliteDatabaseError,
    },
    result::{SqlParserError, SqliteError, SqliteResult},
};

use crate::query::{
    traits::{DistinctProcessing, SqliteStatement},
    SqliteQueryOutcome,
};

#[derive(Debug, Default)]
pub(crate) struct SelectStmt<'a> {
    distinct: Option<Box<dyn DistinctProcessing>>,
    result_columns: Option<ResultColumns<'a>>,
    from: Option<KeywordFrom>,
    // TODO
    origin: Option<TableName<'a>>,
    // TODO
    expr: Option<SqliteExpression>,
}

impl SqliteStatement for SelectStmt<'_> {
    fn run(sql: &str) -> SqliteResult<SqliteQueryOutcome> {
        let cleaned_sql =
            sql.trim()
                .split(';')
                .next()
                .ok_or(SqliteError::SqlParser(SqlParserError(
                    "Invalid SQL. Can't start a query with `;`".into(),
                )))?;

        let mut iter = cleaned_sql.split_whitespace();

        let mut stmt = Self::default();

        while let Some(current_chunk) = iter.next() {
            if stmt.distinct.is_none() {
                if let Some(keyword) = current_chunk.parse::<Keyword>().ok() {
                    if keyword.get().downcast_ref::<Distinct>().is_some() {
                        stmt.distinct = Some(Box::new(Distinct) as Box<dyn DistinctProcessing>);
                    }
                }
                stmt.distinct = Some(Box::new(All) as Box<dyn DistinctProcessing>);
            }

            if stmt.result_columns.is_none() && stmt.distinct.is_some() {
                if let Some(result_columns) = ResultColumns::parse(current_chunk).ok() {
                    stmt.result_columns = Some(result_columns);
                }
            }

            if stmt.from.is_none() && stmt.result_columns.is_some() && stmt.distinct.is_some() {
                if let Some(keyword) = current_chunk.parse::<Keyword>().ok() {
                    if keyword.get().downcast_ref::<KeywordFrom>().is_some() {
                        stmt.from = Some(KeywordFrom);
                    }
                }
            }

            if stmt.origin.is_none()
                && stmt.from.is_some()
                && stmt.result_columns.is_some()
                && stmt.distinct.is_some()
            {
                if let Some(table_name) = TableName::parse(current_chunk).ok() {
                    stmt.origin = Some(table_name);
                }
            }

            if stmt.origin.is_none()
                && stmt.from.is_none()
                && stmt.result_columns.is_none()
                && stmt.distinct.is_some()
            {
                stmt.expr = Some(SqliteExpression::from(current_chunk.to_string()));
            }
        }
        println!("Parsed: {stmt:?}");
        Ok(SqliteQueryOutcome::Failure(SqliteDatabaseError::_Todo))
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
