mod file_header;
mod io;
mod macros;

mod query;
mod result;
mod runtime;
mod traits;

use std::sync::OnceLock;

use runtime::SqliteRuntime;

use crate::result::SqliteResult;

static VERSION_NUMBER: OnceLock<u32> = OnceLock::new();

fn main() -> SqliteResult<()> {
    VERSION_NUMBER.get_or_init(|| {
        let mut s = env!("CARGO_PKG_VERSION").split('.');
        let release = s.next().and_then(|x| x.parse().ok()).unwrap_or(0u32);
        let major = s.next().and_then(|x| x.parse().ok()).unwrap_or(0u32);
        let minor = s.next().and_then(|x| x.parse().ok()).unwrap_or(0u32);

        (10_000 * release) + (100 * major) + minor
    });

    const SMALL_FILE_PATH: &str = "./data/small.sqlite3";

    let mut runtime = SqliteRuntime::start(SMALL_FILE_PATH)?;

    let query = "SELECT * from t1;".parse()?;

    dbg!(&runtime);
    let record = runtime.run_query(query)?;
    dbg!(record);
    // const FLIGHTS_FILE_PATH: &str = "./data/flights-initial.db";
    // let pager2 = Pager::start(FLIGHTS_FILE_PATH);
    // println!("{pager2:X?}");
    Ok(())
}
