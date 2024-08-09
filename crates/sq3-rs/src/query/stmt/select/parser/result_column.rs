use std::marker::PhantomData;

use crate::{
    query::{
        expression::SqliteExpression,
        stmt::select::{parser::result_column, TableName},
    },
    result::{SqlParserError, SqliteError},
    SqliteResult,
};

#[derive(Debug, PartialEq, Eq)]
pub(super) enum ResultColumns<'a> {
    AllColumns,
    AllColumnsDotTable(TableName<'a>),
    Expr(SqliteExpression),
}

impl<'a> ResultColumns<'a> {
    pub fn parse(input: &'a str) -> SqliteResult<Self> {
        let s = input.trim();
        let mut parser = ResultColumnParser::<Initial>::new(s)?;

        if parser.all_columns()?.is_some() {
            let result_column = parser.finish()?;

            dbg!(result_column);
            Ok(ResultColumns::AllColumns)
        } else if parser.table()?.is_some() {
            let result_column = parser.finish()?;
            dbg!(result_column);
            Ok(ResultColumns::AllColumns)
        } else if parser.expr()?.is_some() {
            let result_column = parser.finish()?;
            dbg!(result_column);
            Ok(ResultColumns::AllColumns)
        } else {
            // TODO: Insert Err(..)
            Ok(ResultColumns::AllColumns)
        }
    }
}

// TODO: check visibilty
pub(crate) trait ResultColumnParserState {}

impl ResultColumnParserState for Initial {}

impl ResultColumnParserState for Parsing {}

// States
pub(crate) struct Initial;

pub(crate) struct Parsing;

// Main parser struct
pub(crate) struct ResultColumnParser<'a, State>
where
    State: ResultColumnParserState,
{
    input: &'a str,
    position: usize,
    result_column: Option<ResultColumns<'a>>,
    _state: PhantomData<State>,
}

impl<'a> ResultColumnParser<'a, Initial> {
    pub(crate) fn new(input: &'a str) -> SqliteResult<ResultColumnParser<Parsing>> {
        Ok(ResultColumnParser {
            input,
            position: 0,
            _state: PhantomData,
            result_column: Default::default(),
        })
    }
}

impl<'a, S: ResultColumnParserState> ResultColumnParser<'a, S> {
    fn advance(&mut self, n: usize) {
        self.position += n;
    }

    fn consume_whitespace(&mut self) -> SqliteResult<()> {
        use crate::query::WhiteSpace;
        if self.remaining()?.starts_with(WhiteSpace::as_str()) {
            self.advance(WhiteSpace::len());
            Ok(())
        } else {
            return Err(SqliteError::SqlParser(SqlParserError(format!(
                "Expected `{}`",
                WhiteSpace
            ))));
        }
    }

    fn remaining(&'a self) -> SqliteResult<&'a str> {
        self.input
            .split_at_checked(self.position)
            .map(|(_, remaining)| remaining)
            .ok_or(SqliteError::SqlParser(SqlParserError(
                "Unexpected empty".to_string(),
            )))
    }
}

impl<'a> ResultColumnParser<'a, Parsing> {
    pub(crate) fn all_columns(&mut self) -> SqliteResult<Option<()>> {
        dbg!(self.remaining()?);
        if self.remaining()? == "*" {
            self.result_column = Some(ResultColumns::AllColumns);
            Ok(Some(()))
        } else {
            Ok(None)
        }
    }
    pub(crate) fn table(&mut self) -> SqliteResult<Option<()>> {
        dbg!(self.remaining()?);
        Err(SqliteError::SqlParser(SqlParserError(format!(
            "Not Implemented!"
        ))))
    }
    pub(crate) fn expr(&mut self) -> SqliteResult<Option<()>> {
        dbg!(self.remaining()?);
        Err(SqliteError::SqlParser(SqlParserError(format!(
            "Not Implemented!"
        ))))
    }
    pub(crate) fn finish(mut self) -> SqliteResult<ResultColumns<'a>> {
        dbg!(self.remaining()?);
        self.result_column
            .ok_or(SqliteError::SqlParser(SqlParserError(format!(
                "Impossible State in {} at line {}",
                file!(),
                line!()
            ))))
    }
}
