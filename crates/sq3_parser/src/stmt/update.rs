use std::marker::PhantomData;

use crate::{result::ParserResult, Sq3ParserError};

#[derive(Debug, Default)]
pub(crate) struct UpdateStmt<'a> {
    input: &'a str, // TODO
}

pub(crate) struct UpdateParser<'a, State = Initial>
where
    State: UpdateParserState,
{
    input: &'a str,
    position: usize,
    stmt: UpdateStmt<'a>,
    _state: PhantomData<State>,
}
pub(crate) struct Initial;
pub(crate) struct BeforeUpdate;
pub(crate) struct Complete;

pub(crate) trait UpdateParserState {}

impl UpdateParserState for Initial {}
impl UpdateParserState for BeforeUpdate {}
impl UpdateParserState for Complete {}

impl<'a> UpdateParser<'a, Initial> {
    pub(crate) fn new(input: &'a str) -> UpdateParser<'a, BeforeUpdate> {
        UpdateParser {
            input,
            position: 0,
            _state: PhantomData,
            stmt: Default::default(),
        }
    }
}

impl<'a> UpdateParser<'a, BeforeUpdate> {
    pub(crate) fn update(mut self) -> ParserResult<UpdateParser<'a, Complete>> {
        use crate::keyword::KeywordUpdate;

        // let remaining = Self::get_remaining(self.input, self.position)?;

        // if remaining.starts_with(KeywordUpdate::as_str()) {
        //     self.advance(KeywordUpdate::len());

        //     self.consume_whitespace()?;

        let Self {
            input,
            position,
            stmt,
            ..
        } = self;

        Ok(UpdateParser {
            input,
            position,
            _state: PhantomData,
            stmt,
        })
        // } else {
        //     Err(Sq3ParserError(format!("Expected `{}`", KeywordUpdate)))
        // }
    }
}

impl<'a> UpdateParser<'a, Complete> {
    pub(crate) fn finish(self) -> ParserResult<UpdateStmt<'a>> {
        let Self { stmt, .. } = self;
        Ok(stmt)
    }
}
