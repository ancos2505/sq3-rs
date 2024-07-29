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

    // const SMALL_FILE_PATH: &str = "./data/small.sqlite3";
    const FLIGHTS_FILE_PATH: &str = "./data/flights-initial.db";
    let mut runtime = SqliteRuntime::start(FLIGHTS_FILE_PATH)?;

    run_queries(&mut runtime)?;
    Ok(())
}

fn run_queries(runtime: &mut SqliteRuntime) -> SqliteResult<()> {
    let queries = vec![
        "SELECT id,name FROM users WHERE age > 18",
        "UPDATE users SET name = 'John' WHERE id = 1",
        "INSERT INTO users (name, age) VALUES ('Alice', 30)",
        "DELETE FROM users WHERE id = 5",
        "TRUNCATE TABLE users",
    ];

    for query in queries {
        println!("Query: {}", query);

        match runtime.run_query(query) {
            Ok(parsed_query) => println!("Parsed query: {:#?}", parsed_query),
            Err(e) => println!("Error: {}", e),
        }
    }
    Ok(())
}
