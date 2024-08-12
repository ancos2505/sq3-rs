use std::marker::PhantomData;

use crate::{
    expression::SqliteExpression, result::Sq3ParserError, stmt::select::TableName, ParserResult,
};

#[derive(Debug, PartialEq, Eq)]
pub(super) enum ResultColumns<'a> {
    AllColumns,
    AllColumnsDotTable(TableName<'a>),
    Expr(SqliteExpression),
}

impl<'a> ResultColumns<'a> {
    pub fn parse(input: &'a str) -> ParserResult<Self> {
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
    pub(crate) fn new(input: &'a str) -> ParserResult<ResultColumnParser<Parsing>> {
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

    fn consume_whitespace(&mut self) -> ParserResult<()> {
        use crate::WhiteSpace;
        if self.remaining()?.starts_with(WhiteSpace::as_str()) {
            self.advance(WhiteSpace::len());
            Ok(())
        } else {
            return Err(Sq3ParserError(format!("Expected `{}`", WhiteSpace)));
        }
    }

    fn remaining(&'a self) -> ParserResult<&'a str> {
        self.input
            .split_at_checked(self.position)
            .map(|(_, remaining)| remaining)
            .ok_or(Sq3ParserError("Unexpected empty".to_string()))
    }
}

impl<'a> ResultColumnParser<'a, Parsing> {
    pub(crate) fn all_columns(&mut self) -> ParserResult<Option<()>> {
        dbg!(self.remaining()?);
        if self.remaining()? == "*" {
            self.result_column = Some(ResultColumns::AllColumns);
            Ok(Some(()))
        } else {
            Ok(None)
        }
    }
    pub(crate) fn table(&mut self) -> ParserResult<Option<()>> {
        dbg!(self.remaining()?);
        Err(Sq3ParserError(format!("Not Implemented!")))
    }
    pub(crate) fn expr(&mut self) -> ParserResult<Option<()>> {
        dbg!(self.remaining()?);
        Err(Sq3ParserError(format!("Not Implemented!")))
    }
    pub(crate) fn finish(mut self) -> ParserResult<ResultColumns<'a>> {
        dbg!(self.remaining()?);
        self.result_column.ok_or(Sq3ParserError(format!(
            "Impossible State in {} at line {}",
            file!(),
            line!()
        )))
    }
}
