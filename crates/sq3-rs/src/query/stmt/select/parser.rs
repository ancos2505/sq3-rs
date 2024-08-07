use std::marker::PhantomData;

use crate::{
    result::{SqlParserError, SqliteError},
    SqliteResult,
};

pub(crate) trait ParserState {}

impl ParserState for Initial {}
impl ParserState for AfterSelect {}
impl ParserState for AfterFrom {}
impl ParserState for AfterWhere {}
impl ParserState for Complete {}

// States
pub(crate) struct Initial;
pub(crate) struct AfterSelect;
pub(crate) struct AfterFrom;
pub(crate) struct AfterWhere;
pub(crate) struct Complete;

// Main parser struct
pub(crate) struct SelectParser<State = Initial>
where
    State: ParserState,
{
    input: String,
    position: usize,
    _state: PhantomData<State>,
}

impl<S: ParserState> SelectParser<S> {
    pub(crate) fn new(input: String) -> SelectParser<Initial> {
        SelectParser {
            input,
            position: 0,
            _state: PhantomData,
        }
    }

    fn advance(&mut self, n: usize) {
        self.position += n;
    }

    fn remaining<'a>(&'a self) -> Option<&'a str> {
        self.input
            .split_at_checked(self.position)
            .map(|(_, remaining)| remaining)
    }
}

impl SelectParser<Initial> {
    pub(crate) fn parse_select(mut self) -> SqliteResult<SelectParser<AfterSelect>> {
        let remaining = self
            .remaining()
            .ok_or(SqliteError::SqlParser(SqlParserError(
                "Unexpected empty".to_string(),
            )))?;
        if remaining.starts_with("SELECT ") {
            self.advance(7);
            Ok(SelectParser {
                input: self.input,
                position: self.position,
                _state: PhantomData,
            })
        } else {
            Err(SqliteError::SqlParser(SqlParserError(
                "Expected SELECT".to_string(),
            )))
        }
    }
}

impl SelectParser<AfterSelect> {
    pub(crate) fn parse_columns(mut self) -> SqliteResult<SelectParser<AfterFrom>> {
        let remaining = self
            .remaining()
            .ok_or(SqliteError::SqlParser(SqlParserError(
                "Unexpected empty".to_string(),
            )))?;
        dbg!(&remaining);

        if let Some(pos) = remaining.find("FROM ") {
            self.advance(pos);
            Ok(SelectParser {
                input: self.input,
                position: self.position,
                _state: PhantomData,
            })
        } else {
            Err(SqliteError::SqlParser(SqlParserError(
                "Expected FROM".to_string(),
            )))
        }
    }
}

impl SelectParser<AfterFrom> {
    pub(crate) fn parse_table(mut self) -> SqliteResult<SelectParser<AfterWhere>> {
        let remaining = self
            .remaining()
            .ok_or(SqliteError::SqlParser(SqlParserError(
                "Unexpected empty".to_string(),
            )))?;
        dbg!(&remaining);

        if let Some(pos) = remaining.find("WHERE ") {
            self.advance(pos);
            Ok(SelectParser {
                input: self.input,
                position: self.position,
                _state: PhantomData,
            })
        } else {
            Err(SqliteError::SqlParser(SqlParserError(
                "Expected WHERE".to_string(),
            )))
        }
    }
}

impl SelectParser<AfterWhere> {
    pub(crate) fn parse_condition(mut self) -> SqliteResult<SelectParser<Complete>> {
        let remaining = self
            .remaining()
            .ok_or(SqliteError::SqlParser(SqlParserError(
                "Unexpected empty".to_string(),
            )))?;
        dbg!(&remaining);

        self.advance(remaining.len());

        Ok(SelectParser {
            input: self.input,
            position: self.position,
            _state: PhantomData,
        })
    }
}
