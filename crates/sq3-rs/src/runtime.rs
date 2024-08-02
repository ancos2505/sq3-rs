mod pager;

use std::num::NonZeroU32;

use crate::{
    query::{SqliteDatabaseError, SqliteQuery, SqliteQueryOutcome, SqliteRecord},
    result::SqliteResult,
};

use self::pager::Pager;

#[derive(Debug)]
pub(crate) struct SqliteRuntime {
    pager: Pager,
}

impl SqliteRuntime {
    pub fn start<S: AsRef<str>>(conn_str: S) -> SqliteResult<Self> {
        let pager = Pager::start(conn_str)?;

        println!("{pager:X?}");

        Ok(Self { pager })
    }
    pub fn run_mockup(&mut self) -> SqliteResult<SqliteRecord> {
        let p1 = self.pager.get_page(NonZeroU32::new(1).unwrap())?;
        // println!("{p1:X?}");

        let p2 = self.pager.get_page(NonZeroU32::new(2).unwrap())?;
        // println!("{p2:X?}");

        let p3 = self.pager.get_page(NonZeroU32::new(3).unwrap())?;
        // println!("{p3:X?}");

        let p4 = self.pager.get_page(NonZeroU32::new(4).unwrap())?;
        // println!("{p4:X?}");

        Ok(Default::default())
    }
    pub fn run_query(&mut self, query_str: &str) -> SqliteResult<SqliteRecord> {
        let sqlite_outcome = SqliteQuery::run(query_str)?;

        Ok(sqlite_outcome.into())
    }
}
