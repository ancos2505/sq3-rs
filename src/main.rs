mod file_header;
mod io;
mod macros;
mod pager;
mod result;
mod traits;

use std::{num::NonZeroU32, sync::OnceLock};

use crate::{pager::Pager, result::SqliteResult};

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
    let mut pager1 = Pager::start(SMALL_FILE_PATH)?;

    println!("{pager1:X?}");

    let p1 = pager1.get_page(NonZeroU32::new(1).unwrap())?;
    println!("{p1:X?}");

    let p2 = pager1.get_page(NonZeroU32::new(2).unwrap())?;
    println!("{p2:X?}");

    let p3 = pager1.get_page(NonZeroU32::new(3).unwrap())?;
    println!("{p3:X?}");

    let p4 = pager1.get_page(NonZeroU32::new(4).unwrap())?;
    println!("{p4:X?}");

    // const FLIGHTS_FILE_PATH: &str = "./data/flights-initial.db";
    // let pager2 = Pager::start(FLIGHTS_FILE_PATH);
    // println!("{pager2:X?}");
    Ok(())
}
