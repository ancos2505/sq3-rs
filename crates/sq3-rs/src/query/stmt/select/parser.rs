use std::marker::PhantomData;

use crate::{
    result::{SqlParserError, SqliteError},
    SqliteResult,
};

use super::SelectStmt;

pub(crate) trait ParserState {}

impl ParserState for Initial {}
//impl ParserState for AfterWhiteSpace {}
impl ParserState for AfterSelect {}
impl ParserState for AfterDistinct {}
impl ParserState for AfterResultColumns {}
impl ParserState for AfterFrom {}
impl ParserState for AfterWhere {}
impl ParserState for Complete {}

// States
pub(crate) struct Initial;
pub(crate) struct AfterSelect;

// pub(crate) struct AfterWhiteSpace;

pub(crate) struct AfterDistinct;

pub(crate) struct AfterResultColumns;

pub(crate) struct AfterFrom;
pub(crate) struct AfterWhere;
pub(crate) struct Complete;

// Main parser struct
pub(crate) struct SelectParser<'a, State = Initial>
where
    State: ParserState,
{
    input: &'a str,
    position: usize,
    stmt: SelectStmt<'a>,
    _state: PhantomData<State>,
}

impl<'a, S: ParserState> SelectParser<'a, S> {
    pub(crate) fn new(input: &'a str) -> SelectParser<Initial> {
        SelectParser {
            input,
            position: 0,
            _state: PhantomData,
            stmt: Default::default(),
        }
    }

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

impl<'a> SelectParser<'a, Initial> {
    pub(crate) fn parse_select(mut self) -> SqliteResult<SelectParser<'a, AfterSelect>> {
        use crate::query::keyword::Select;
        let remaining = self.remaining()?;
        dbg!(&remaining);

        if remaining.starts_with(Select::as_str()) {
            self.advance(Select::len());

            self.consume_whitespace()?;

            let Self {
                input,
                position,
                stmt,
                ..
            } = self;

            Ok(SelectParser {
                input,
                position,
                _state: PhantomData,
                stmt,
            })
        } else {
            Err(SqliteError::SqlParser(SqlParserError(format!(
                "Expected `{}`",
                Select
            ))))
        }
    }
}

impl<'a> SelectParser<'a, AfterSelect> {
    pub(crate) fn parse_distinct(mut self) -> SqliteResult<SelectParser<'a, AfterDistinct>> {
        use crate::query::stmt::select::KeywordFrom;
        let remaining = self.remaining()?;
        dbg!(&remaining);

        if let Some(pos) = remaining.find(KeywordFrom::as_str()) {
            self.advance(pos);

            self.consume_whitespace()?;

            let Self {
                input,
                position,
                stmt,
                ..
            } = self;
            Ok(SelectParser {
                input,
                position,
                _state: PhantomData,
                stmt,
            })
        } else {
            Err(SqliteError::SqlParser(SqlParserError(format!(
                "Expected `{}`",
                KeywordFrom
            ))))
        }
    }
}


impl<'a> SelectParser<'a, AfterDistinct> {
    pub(crate) fn parse_columns(mut self) -> SqliteResult<SelectParser<'a, AfterFrom>> {
        use crate::query::stmt::select::KeywordFrom;
        let remaining = self.remaining()?;
        dbg!(&remaining);

        if let Some(pos) = remaining.find(KeywordFrom::as_str()) {
            self.advance(KeywordFrom::len());

            self.consume_whitespace()?;

            let Self {
                input,
                position,
                stmt,
                ..
            } = self;
            Ok(SelectParser {
                input,
                position,
                _state: PhantomData,
                stmt,
            })
        } else {
            Err(SqliteError::SqlParser(SqlParserError(format!(
                "Expected `{}`",
                KeywordFrom
            ))))
        }
    }
}


impl<'a> SelectParser<'a, AfterFrom> {
    pub(crate) fn parse_table(mut self) -> SqliteResult<SelectParser<'a, AfterWhere>> {
        use crate::query::stmt::select::KeywordWhere;
        let remaining = self.remaining()?;
        dbg!(&remaining);

        if let Some(pos) = remaining.find(KeywordWhere::as_str()) {
            self.advance(KeywordWhere::len());

            self.consume_whitespace()?;

            let Self {
                input,
                position,
                stmt,
                ..
            } = self;
            Ok(SelectParser {
                input,
                position,
                stmt,
                _state: PhantomData,
            })
        } else {
            Err(SqliteError::SqlParser(SqlParserError(format!(
                "Expected `{}`",
                KeywordWhere
            ))))
        }
    }
}

impl<'a> SelectParser<'a, AfterWhere> {
    pub(crate) fn parse_condition(mut self) -> SqliteResult<SelectParser<'a, Complete>> {
        let remaining = self.remaining()?;
        dbg!(&remaining);

        self.advance(remaining.len());

        let Self {
            input,
            position,
            stmt,
            ..
        } = self;
        Ok(SelectParser {
            input,
            position,
            stmt,
            _state: PhantomData,
        })
    }
}
