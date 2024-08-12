#[cfg(test)]
mod tests;

mod result_column;

use std::marker::PhantomData;

use crate::{result::Sq3ParserError, ParserResult};

use super::SelectStmt;

pub(crate) trait SelectParserState {}

impl SelectParserState for Initial {}
impl SelectParserState for BeforeSelect {}
impl SelectParserState for AfterSelect {}
impl SelectParserState for AfterDistinct {}
impl SelectParserState for AfterResultColumns {}
impl SelectParserState for AfterFrom {}
impl SelectParserState for AfterWhere {}
impl SelectParserState for Complete {}

// States
pub(crate) struct Initial;

pub(crate) struct BeforeSelect;
pub(crate) struct AfterSelect;

pub(crate) struct AfterDistinct;

pub(crate) struct AfterResultColumns;

pub(crate) struct AfterFrom;
pub(crate) struct AfterWhere;
pub(crate) struct Complete;

// Main parser struct
pub(crate) struct SelectParser<'a, State = Initial>
where
    State: SelectParserState,
{
    input: &'a str,
    position: usize,
    stmt: SelectStmt<'a>,
    _state: PhantomData<State>,
}

impl<'a, S: SelectParserState> SelectParser<'a, S> {
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

impl<'a> SelectParser<'a, Initial> {
    pub(crate) fn new(input: &'a str) -> SelectParser<'a, BeforeSelect> {
        SelectParser {
            input,
            position: 0,
            _state: PhantomData,
            stmt: Default::default(),
        }
    }
}

impl<'a> SelectParser<'a, BeforeSelect> {
    pub(crate) fn select(mut self) -> ParserResult<SelectParser<'a, AfterSelect>> {
        use crate::keyword::KeywordSelect;
        dbg!(self.remaining()?);

        if self.remaining()?.starts_with(KeywordSelect::as_str()) {
            self.advance(KeywordSelect::len());

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
            Err(Sq3ParserError(format!("Expected `{}`", KeywordSelect)))
        }
    }
}

impl<'a> SelectParser<'a, AfterSelect> {
    pub(crate) fn distinct(mut self) -> ParserResult<SelectParser<'a, AfterDistinct>> {
        use crate::{
            keyword::{KeywordAll, KeywordDistinct},
            stmt::select::KeywordFrom,
        };

        dbg!(self.remaining()?);

        if self.remaining()?.find(KeywordFrom::as_str()).is_some() {
            if self.remaining()?.starts_with(KeywordDistinct::as_str()) {
                self.advance(KeywordDistinct::len());
                self.consume_whitespace()?;
                let Self {
                    input,
                    position,
                    mut stmt,
                    ..
                } = self;

                stmt.distinct = Some(Box::new(KeywordDistinct));

                Ok(SelectParser {
                    input,
                    position,
                    _state: PhantomData,
                    stmt,
                })
            } else if self.remaining()?.starts_with(KeywordAll::as_str()) {
                self.advance(KeywordAll::len());
                self.consume_whitespace()?;
                let Self {
                    input,
                    position,
                    mut stmt,
                    ..
                } = self;

                stmt.distinct = Some(Box::new(KeywordAll));

                Ok(SelectParser {
                    input,
                    position,
                    _state: PhantomData,
                    stmt,
                })
            } else {
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
            }
        } else {
            Err(Sq3ParserError(format!("Expected `{}`", KeywordFrom)))
        }
    }
}

impl<'a> SelectParser<'a, AfterDistinct> {
    pub(crate) fn result_columns(mut self) -> ParserResult<SelectParser<'a, AfterFrom>> {
        use self::result_column::ResultColumns;
        use crate::stmt::select::KeywordFrom;
        dbg!(self.remaining()?);

        if let Some(pos) = self.remaining()?.find(KeywordFrom::as_str()) {
            let (s, _) = self.remaining()?.split_at(pos);
            dbg!(s);
            let parsed_result_columns = ResultColumns::parse(s)?;
            dbg!(&parsed_result_columns);
            self.advance(pos);

            // self.consume_whitespace()?;

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
            Err(Sq3ParserError(format!("Expected `{}`", KeywordFrom)))
        }
    }
}

impl<'a> SelectParser<'a, AfterFrom> {
    pub(crate) fn table(mut self) -> ParserResult<SelectParser<'a, AfterWhere>> {
        use crate::stmt::select::KeywordWhere;
        dbg!(self.remaining()?);

        if let Some(pos) = self.remaining()?.find(KeywordWhere::as_str()) {
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
            Err(Sq3ParserError(format!("Expected `{}`", KeywordWhere)))
        }
    }
}

impl<'a> SelectParser<'a, AfterWhere> {
    pub(crate) fn condition(mut self) -> ParserResult<SelectParser<'a, Complete>> {
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
