mod file_header;
mod io;
mod macros;
mod result;
mod runtime;
mod traits;

use std::sync::OnceLock;

use sq3_parser::SqliteRecord;

use crate::runtime::SqliteRuntime;

pub use crate::result::SqliteResult;

static VERSION_NUMBER: OnceLock<u32> = OnceLock::new();

#[derive(Debug)]
pub struct SqliteConnection {
    runtime: SqliteRuntime,
}

impl SqliteConnection {
    pub fn connect<S: AsRef<str>>(conn_str: S) -> SqliteResult<Self> {
        bootstrap();
        let runtime = SqliteRuntime::start(conn_str)?;
        Ok(Self { runtime })
    }
    pub fn run_query(&mut self, query_str: &str) -> SqliteResult<SqliteRecord> {
        self.runtime.run_query(query_str)
    }
}

fn bootstrap() {
    VERSION_NUMBER.get_or_init(|| {
        let mut s = env!("CARGO_PKG_VERSION").split('.');
        let release = s.next().and_then(|x| x.parse().ok()).unwrap_or(0u32);
        let major = s.next().and_then(|x| x.parse().ok()).unwrap_or(0u32);
        let minor = s.next().and_then(|x| x.parse().ok()).unwrap_or(0u32);

        (10_000 * release) + (100 * major) + minor
    });
}
