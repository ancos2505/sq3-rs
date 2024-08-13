use std::marker::PhantomData;

use crate::result::ParserResult;

#[derive(Debug, Default)]
pub(crate) struct DeleteStmt<'a> {
    input: &'a str, // TODO
}

pub(crate) struct DeleteParser<'a, State = Initial>
where
    State: DeleteParserState,
{
    input: &'a str,
    position: usize,
    stmt: DeleteStmt<'a>,
    _state: PhantomData<State>,
}
pub(crate) struct Initial;
pub(crate) struct BeforeDelete;
pub(crate) struct Complete;

pub(crate) trait DeleteParserState {}

impl DeleteParserState for Initial {}
impl DeleteParserState for BeforeDelete {}
impl DeleteParserState for Complete {}

impl<'a> DeleteParser<'a, Initial> {
    pub(crate) fn new(input: &'a str) -> DeleteParser<'a, BeforeDelete> {
        DeleteParser {
            input,
            position: 0,
            _state: PhantomData,
            stmt: Default::default(),
        }
    }
}

impl<'a> DeleteParser<'a, BeforeDelete> {
    pub(crate) fn delete(self) -> ParserResult<DeleteParser<'a, Complete>> {
        // use crate::keyword::KeywordDelete;

        // let remaining = Self::get_remaining(self.input, self.position)?;

        // if remaining.starts_with(KeywordDelete::as_str()) {
        //     self.advance(KeywordDelete::len());

        //     self.consume_whitespace()?;

        let Self {
            input,
            position,
            stmt,
            ..
        } = self;

        Ok(DeleteParser {
            input,
            position,
            _state: PhantomData,
            stmt,
        })
        // } else {
        //     Err(Sq3ParserError(format!("Expected `{}`", KeywordDelete)))
        // }
    }
}

impl<'a> DeleteParser<'a, Complete> {
    pub(crate) fn finish(self) -> ParserResult<DeleteStmt<'a>> {
        let Self { stmt, .. } = self;
        Ok(stmt)
    }
}
