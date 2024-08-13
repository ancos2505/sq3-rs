use std::marker::PhantomData;

use crate::result::ParserResult;

#[derive(Debug, Default)]
pub(crate) struct ExplainStmt<'a> {
    input: &'a str, // TODO
}

#[derive(Debug)]
pub(crate) struct ExplainParser<'a, State = Initial>
where
    State: ExplainParserState,
{
    input: &'a str,
    position: usize,
    stmt: ExplainStmt<'a>,
    _state: PhantomData<State>,
}
pub(crate) struct Initial;
pub(crate) struct BeforeExplain;
pub(crate) struct Complete;

pub(crate) trait ExplainParserState {}

impl ExplainParserState for Initial {}
impl ExplainParserState for BeforeExplain {}
impl ExplainParserState for Complete {}

impl<'a> ExplainParser<'a, Initial> {
    pub(crate) fn new(input: &'a str) -> ExplainParser<'a, BeforeExplain> {
        ExplainParser {
            input,
            position: 0,
            _state: PhantomData,
            stmt: Default::default(),
        }
    }
}

impl<'a> ExplainParser<'a, BeforeExplain> {
    pub(crate) fn explain(self) -> ParserResult<ExplainParser<'a, Complete>> {
        // use crate::keyword::KeywordExplain;

        // let remaining = Self::get_remaining(self.input, self.position)?;

        // if remaining.starts_with(KeywordExplain::as_str()) {
        //     self.advance(KeywordExplain::len());

        //     self.consume_whitespace()?;

        let Self {
            input,
            position,
            stmt,
            ..
        } = self;

        Ok(ExplainParser {
            input,
            position,
            _state: PhantomData,
            stmt,
        })
        // } else {
        //     Err(Sq3ParserError(format!("Expected `{}`", KeywordExplain)))
        // }
    }
}

impl<'a> ExplainParser<'a, Complete> {
    pub(crate) fn finish(self) -> ParserResult<ExplainStmt<'a>> {
        let Self { stmt, .. } = self;
        Ok(stmt)
    }
}
