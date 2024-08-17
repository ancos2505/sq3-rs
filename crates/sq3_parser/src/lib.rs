mod explain;
mod expression;
mod helpers;
mod keyword;
mod literal_value;
mod result;
mod stmt;
mod traits;
mod white_space;

#[cfg(test)]
mod tests;

use explain::ExplainParser;
use keyword::{
    KeywordDelete, KeywordExplain, KeywordInsert, KeywordSelect, KeywordUpdate, KeywordWith,
};
use stmt::{DeleteParser, InsertParser, SelectParser, UpdateParser};

use crate::{helpers::ASCII_WHITESPACE_CHAR, keyword::Keyword};

use crate::result::ParserResult;

pub use crate::{result::Sq3ParserError, traits::TypeName};

pub(self) use self::white_space::WhiteSpace;

/// ## SqliteQuery
///
/// **Reference:** https://www.sqlite.org/syntaxdiagrams.html#sql-stmt
///
#[derive(Debug)]
pub struct SqliteQuery;

impl SqliteQuery {
    pub fn parse(sql: &str) -> ParserResult<SqliteQuery> {
        if !sql.ends_with(";") {
            return Err(Sq3ParserError(
                "Invalid query. Reason: Every query must ends with `;`.".into(),
            ));
        }

        ///////////////////////////////////////

        let (current_to_parse, next_to_parse) = sql
            .trim()
            .split_once(ASCII_WHITESPACE_CHAR)
            .ok_or(Sq3ParserError(
                "Invalid query. Reason: There is no whitespaces.".into(),
            ))?;

        let keyword = current_to_parse.parse::<Keyword>()?;

        if keyword.get().downcast_ref::<KeywordExplain>().is_some() {
            return Err(Sq3ParserError(format!(
                "{} is not supported.",
                KeywordExplain
            )));
        }

        if keyword.get().downcast_ref::<KeywordWith>().is_some() {
            return Err(Sq3ParserError(format!("{} is not supported.", KeywordWith)));
        }

        if keyword.get().downcast_ref::<KeywordSelect>().is_some() {
            let parser = SelectParser::new(next_to_parse)
                .select()
                .and_then(|p| p.distinct())
                .and_then(|p| p.result_columns())
                .and_then(|p| p.from())
                .and_then(|p| p.table())
                .and_then(|p| p.r#where())
                .and_then(|p| p.condition())?;
            {
                let stmt = parser.finish()?;
                dbg!(stmt);
            }

            Ok(Self)
        } else if keyword.get().downcast_ref::<KeywordInsert>().is_some() {
            let stmt = InsertParser::new(next_to_parse)
                .insert()
                .and_then(|p| p.finish())?;
            dbg!(stmt);
            Ok(Self)
        } else if keyword.get().downcast_ref::<KeywordUpdate>().is_some() {
            let stmt = UpdateParser::new(next_to_parse)
                .update()
                .and_then(|p| p.finish())?;
            dbg!(stmt);
            Ok(Self)
        } else if keyword.get().downcast_ref::<KeywordDelete>().is_some() {
            let stmt = DeleteParser::new(next_to_parse)
                .delete()
                .and_then(|p| p.finish())?;
            dbg!(stmt);
            Ok(Self)
        } else if keyword.get().downcast_ref::<KeywordExplain>().is_some() {
            let stmt = ExplainParser::new(next_to_parse)
                .explain()
                .and_then(|p| p.finish())?;
            dbg!(stmt);
            Ok(Self)
        } else {
            // TODO
            Err(Sq3ParserError(format!(
                "Can't found valid query for {:?}",
                current_to_parse
            )))
        }
    }
}
