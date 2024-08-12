mod explain;
mod expression;
mod helpers;
mod keyword;
mod literal_value;
mod result;
mod router;
mod stmt;
mod traits;
mod white_space;

#[cfg(test)]
mod tests;

use self::router::QueryRouter;
use crate::result::ParserResult;

pub use crate::{
    helpers::{SqliteDatabaseError, SqliteQueryOutcome, SqliteRecord},
    result::Sq3ParserError,
    traits::TypeName,
};

pub(self) use self::white_space::WhiteSpace;

/// ## SqliteQuery
///
/// **Reference:** https://www.sqlite.org/syntaxdiagrams.html#sql-stmt
///
#[derive(Debug)]
pub struct SqliteQuery;

impl SqliteQuery {
    pub fn run(sql: &str) -> ParserResult<SqliteQueryOutcome> {
        if !sql.ends_with(";") {
            return Err(Sq3ParserError(
                "Invalid query. Reason: Every query must ends with `;`.".into(),
            ));
        }

        let db_outcome = QueryRouter::run(sql)?;

        Ok(db_outcome)
    }
}
