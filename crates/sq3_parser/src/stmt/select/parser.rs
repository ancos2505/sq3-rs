#[cfg(test)]
mod tests;

mod result_column;

use std::marker::PhantomData;

use crate::{
    result::Sq3ParserError,
    stmt::{select::TableName, SqliteStmt},
    ParserResult, SqliteQuery,
};

use super::SelectStmt;

pub(crate) use self::result_column::ResultColumns;

pub(crate) trait SelectParserState {}

impl SelectParserState for Initial {}
impl SelectParserState for BeforeSelect {}
impl SelectParserState for AfterSelect {}
impl SelectParserState for AfterDistinct {}
impl SelectParserState for AfterResultColumns {}
impl SelectParserState for AfterFrom {}
impl SelectParserState for AfterTableName {}
impl SelectParserState for AfterWhere {}
impl SelectParserState for Complete {}

// States
pub(crate) struct Initial;

pub(crate) struct BeforeSelect;
pub(crate) struct AfterSelect;

pub(crate) struct AfterDistinct;

pub(crate) struct AfterResultColumns;

pub(crate) struct AfterFrom;

pub(crate) struct AfterTableName;
pub(crate) struct AfterWhere;
pub(crate) struct Complete;

// Main parser struct
pub(crate) struct SelectParser<'a, State = Initial>
where
    State: SelectParserState,
{
    position: usize,
    stmt: Box<SelectStmt<'a>>,
    _state: PhantomData<State>,
}

impl<'a, S: SelectParserState> SelectParser<'a, S> {
    fn advance(&mut self, n: usize) {
        self.position += n;
    }

    fn consume_whitespace(&mut self) -> ParserResult<()> {
        use crate::WhiteSpace;
        if Self::get_remaining(self.stmt.input, self.position)?.starts_with(WhiteSpace::as_str()) {
            self.advance(WhiteSpace::len());
            Ok(())
        } else {
            return Err(Sq3ParserError(format!("Expected `{}`", WhiteSpace)));
        }
    }

    fn get_remaining(input: &str, pos: usize) -> ParserResult<&str> {
        input
            .split_at_checked(pos)
            .map(|(_, remaining)| remaining)
            .ok_or(Sq3ParserError("Unexpected empty".to_string()))
    }
}

impl<'a> SelectParser<'a, Initial> {
    pub(crate) fn new(input: &'a str) -> SelectParser<'a, BeforeSelect> {
        SelectParser {
            position: 0,
            _state: PhantomData,
            stmt: Box::new(SelectStmt {
                input,
                ..Default::default()
            }),
        }
    }
}

impl<'a> SelectParser<'a, BeforeSelect> {
    pub(crate) fn select(mut self) -> ParserResult<SelectParser<'a, AfterSelect>> {
        use crate::keyword::KeywordSelect;

        let remaining = Self::get_remaining(self.stmt.input, self.position)?;

        if remaining.starts_with(KeywordSelect::as_str()) {
            self.advance(KeywordSelect::len());

            self.consume_whitespace()?;

            let Self { position, stmt, .. } = self;

            Ok(SelectParser {
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

        let remaining = Self::get_remaining(self.stmt.input, self.position)?;

        if remaining.find(KeywordFrom::as_str()).is_some() {
            if remaining.starts_with(KeywordDistinct::as_str()) {
                self.advance(KeywordDistinct::len());
                self.consume_whitespace()?;
                let Self {
                    position, mut stmt, ..
                } = self;

                stmt.distinct = Some(Box::new(KeywordDistinct));

                Ok(SelectParser {
                    position,
                    _state: PhantomData,
                    stmt,
                })
            } else if remaining.starts_with(KeywordAll::as_str()) {
                self.advance(KeywordAll::len());
                self.consume_whitespace()?;
                let Self {
                    position, mut stmt, ..
                } = self;

                stmt.distinct = Some(Box::new(KeywordAll));

                Ok(SelectParser {
                    position,
                    _state: PhantomData,
                    stmt,
                })
            } else {
                let Self {
                    position, mut stmt, ..
                } = self;
                stmt.distinct = Some(Box::new(KeywordAll));

                Ok(SelectParser {
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
    pub(crate) fn result_columns(mut self) -> ParserResult<SelectParser<'a, AfterResultColumns>> {
        use self::result_column::ResultColumns;
        use crate::stmt::select::KeywordFrom;

        let remaining = Self::get_remaining(self.stmt.input, self.position)?;

        if let Some(pos) = remaining.find(KeywordFrom::as_str()) {
            let maybe_result_columns = remaining
                .split_at_checked(pos)
                .map(|(s, _)| ResultColumns::parse(s).ok())
                .flatten();

            self.advance(pos);

            self.stmt.result_columns = maybe_result_columns;
            Ok(SelectParser {
                position: self.position,
                _state: PhantomData,
                stmt: self.stmt,
            })
        } else {
            Err(Sq3ParserError(format!("Expected `{}`", KeywordFrom)))
        }
    }
}

impl<'a> SelectParser<'a, AfterResultColumns> {
    pub(crate) fn from(mut self) -> ParserResult<SelectParser<'a, AfterFrom>> {
        use crate::keyword::KeywordFrom;

        let remaining = Self::get_remaining(self.stmt.input, self.position)?;

        if remaining.starts_with(KeywordFrom::as_str()) {
            self.advance(KeywordFrom::len());
            self.consume_whitespace()?;
            let Self {
                position, mut stmt, ..
            } = self;

            stmt.from = Some(KeywordFrom);

            Ok(SelectParser {
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
    pub(crate) fn table(mut self) -> ParserResult<SelectParser<'a, AfterTableName>> {
        use crate::stmt::select::KeywordWhere;
        let remaining = Self::get_remaining(self.stmt.input, self.position)?;

        

        if let Some(pos) = remaining.find(KeywordWhere::as_str()) {
            let maybe_table_name = remaining
                .split_at_checked(pos)
                .map(|(s, _)| TableName::parse(s).ok())
                .flatten();
            self.advance(pos);

            let Self {
                position, mut stmt, ..
            } = self;
            stmt.origin = maybe_table_name;
            Ok(SelectParser {
                position,
                stmt,
                _state: PhantomData,
            })
        } else if let Some(pos) = remaining.find(";") {
            let maybe_table_name = remaining
                .split_at_checked(pos)
                .map(|(s, _)| {
                    let res = TableName::parse(s);
                    
                    res.ok()
                })
                .flatten();
            self.advance(pos);

            let Self {
                position, mut stmt, ..
            } = self;
            stmt.origin = maybe_table_name;
            Ok(SelectParser {
                position,
                stmt,
                _state: PhantomData,
            })
        } else {
            // Err(Sq3ParserError(format!("Expected `{}`", KeywordWhere)))
            let Self { position, stmt, .. } = self;
            Ok(SelectParser {
                position,
                _state: PhantomData,
                stmt,
            })
        }
    }
}

impl<'a> SelectParser<'a, AfterTableName> {
    pub(crate) fn r#where(mut self) -> ParserResult<SelectParser<'a, AfterWhere>> {
        use crate::keyword::KeywordWhere;

        let remaining = Self::get_remaining(self.stmt.input, self.position)?;

        if remaining.starts_with(KeywordWhere::as_str()) {
            self.advance(KeywordWhere::len());
            self.consume_whitespace()?;
            let Self {
                position, mut stmt, ..
            } = self;

            stmt.r#where = Some(KeywordWhere);

            Ok(SelectParser {
                position,
                _state: PhantomData,
                stmt,
            })
        } else {
            let Self { position, stmt, .. } = self;

            Ok(SelectParser {
                position,
                _state: PhantomData,
                stmt,
            })
            // Err(Sq3ParserError(format!("Expected `{}`", KeywordWhere)))
        }
    }
}

impl<'a> SelectParser<'a, AfterWhere> {
    pub(crate) fn condition(mut self) -> ParserResult<SelectParser<'a, Complete>> {
        let remaining = Self::get_remaining(self.stmt.input, self.position)?;

        
        // TODO
        self.advance(remaining.len());

        let Self { position, stmt, .. } = self;
        Ok(SelectParser {
            position,
            stmt,
            _state: PhantomData,
        })
    }
}

impl<'a> SelectParser<'a, Complete> {
    pub(crate) fn finish(&self) -> ParserResult<&Box<SelectStmt>> {
        Ok(&self.stmt)
    }
}
