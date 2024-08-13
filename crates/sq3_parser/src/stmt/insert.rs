use std::marker::PhantomData;

use crate::result::ParserResult;

#[derive(Debug, Default)]
pub(crate) struct InsertStmt<'a> {
    input: &'a str, // TODO
}

pub(crate) struct InsertParser<'a, State = Initial>
where
    State: InsertParserState,
{
    input: &'a str,
    position: usize,
    stmt: InsertStmt<'a>,
    _state: PhantomData<State>,
}
pub(crate) struct Initial;
pub(crate) struct BeforeInsert;
pub(crate) struct Complete;

pub(crate) trait InsertParserState {}

impl InsertParserState for Initial {}
impl InsertParserState for BeforeInsert {}
impl InsertParserState for Complete {}

impl<'a> InsertParser<'a, Initial> {
    pub(crate) fn new(input: &'a str) -> InsertParser<'a, BeforeInsert> {
        InsertParser {
            input,
            position: 0,
            _state: PhantomData,
            stmt: Default::default(),
        }
    }
}

impl<'a> InsertParser<'a, BeforeInsert> {
    pub(crate) fn insert(self) -> ParserResult<InsertParser<'a, Complete>> {
        // use crate::keyword::KeywordInsert;

        // let remaining = Self::get_remaining(self.input, self.position)?;

        // if remaining.starts_with(KeywordInsert::as_str()) {
        //     self.advance(KeywordInsert::len());

        //     self.consume_whitespace()?;

        let Self {
            input,
            position,
            stmt,
            ..
        } = self;

        Ok(InsertParser {
            input,
            position,
            _state: PhantomData,
            stmt,
        })
        // } else {
        //     Err(Sq3ParserError(format!("Expected `{}`", KeywordInsert)))
        // }
    }
}

impl<'a> InsertParser<'a, Complete> {
    pub(crate) fn finish(self) -> ParserResult<InsertStmt<'a>> {
        let Self { stmt, .. } = self;
        Ok(stmt)
    }
}
