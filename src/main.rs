mod file_header;
mod log;
mod log_macros;
mod result;
mod sqlite_cli;
mod traits;

use std::{fs::File, io::Read, path::Path};

use file_header::PageSize;
use result::SqliteResult;

use crate::file_header::SqliteHeader;

#[derive(Debug)]
struct Page<const N: usize> {
    size: usize,
    data: [u8; N],
}

pub trait ValidPage {
    fn data_owned(&self) -> Vec<u8>;
}

impl ValidPage for Page<512> {
    fn data_owned(&self) -> Vec<u8> {
        self.data.to_vec()
    }
}
impl ValidPage for Page<1024> {
    fn data_owned(&self) -> Vec<u8> {
        self.data.to_vec()
    }
}
impl ValidPage for Page<2048> {
    fn data_owned(&self) -> Vec<u8> {
        self.data.to_vec()
    }
}
impl ValidPage for Page<4096> {
    fn data_owned(&self) -> Vec<u8> {
        self.data.to_vec()
    }
}
impl ValidPage for Page<8192> {
    fn data_owned(&self) -> Vec<u8> {
        self.data.to_vec()
    }
}
impl ValidPage for Page<16384> {
    fn data_owned(&self) -> Vec<u8> {
        self.data.to_vec()
    }
}
impl ValidPage for Page<32768> {
    fn data_owned(&self) -> Vec<u8> {
        self.data.to_vec()
    }
}
impl ValidPage for Page<65536> {
    fn data_owned(&self) -> Vec<u8> {
        self.data.to_vec()
    }
}

impl<const N: usize> Page<N> {
    pub fn new() -> Self {
        Self {
            size: N,
            data: [0; N],
        }
    }
    pub fn parse<P: AsRef<Path>>(path: P) -> Self {
        let mut file_small = File::open(path).unwrap();
        let mut buf: [u8; N] = [0; N];
        let bytes_read = file_small.read(&mut buf).unwrap();
        dbg!(bytes_read);

        Self { size: N, data: buf }
    }

    fn size(&self) -> usize {
        self.size
    }

    fn data(&self) -> [u8; N] {
        self.data
    }
}

#[derive(Debug)]
struct Pager {
    file_header: SqliteHeader,
    current_page: Option<Vec<u8>>,
}

impl Pager {
    pub fn start<P: AsRef<Path>>(path: P) -> Self {
        let page_size = Self::get_page_size(&path);
        let data = match &page_size {
            PageSize::L512 => {
                const PAGE_SIZE: usize = 512;
                let page = Page::<PAGE_SIZE>::parse(path);
                println!("{page:X?}");
                Self::get_data(page)
            }
            PageSize::L1024 => {
                const PAGE_SIZE: usize = 1024;
                let page = Page::<PAGE_SIZE>::parse(path);
                println!("{page:X?}");
                Self::get_data(page)
            }
            PageSize::L2048 => {
                const PAGE_SIZE: usize = 2048;
                let page = Page::<PAGE_SIZE>::parse(path);
                println!("{page:X?}");
                Self::get_data(page)
            }
            PageSize::L4096 => {
                const PAGE_SIZE: usize = 4096;
                let page = Page::<PAGE_SIZE>::parse(path);
                println!("{page:X?}");
                Self::get_data(page)
            }
            PageSize::L8192 => {
                const PAGE_SIZE: usize = 8192;
                let page = Page::<PAGE_SIZE>::parse(path);
                println!("{page:X?}");
                Self::get_data(page)
            }
            PageSize::L16384 => {
                const PAGE_SIZE: usize = 16384;
                let page = Page::<PAGE_SIZE>::parse(path);
                println!("{page:X?}");
                Self::get_data(page)
            }
            PageSize::L32768 => {
                const PAGE_SIZE: usize = 32768;
                let page = Page::<PAGE_SIZE>::parse(path);
                println!("{page:X?}");
                Self::get_data(page)
            }
            PageSize::L65536 => {
                const PAGE_SIZE: usize = 65536;
                let page = Page::<PAGE_SIZE>::parse(path);
                println!("{page:X?}");
                Self::get_data(page)
            }
        };
        // TODO
        Self {
            current_page: Some(data),
            page_size,
        }
    }
    fn get_page_size<P: AsRef<Path>>(path: P) -> SqliteResult<PageSize> {
        let mut file = File::open(path).unwrap();
        let mut buf: [u8; 18] = [0; 18];
        let bytes_read = file.read(&mut buf).unwrap();
        dbg!(&bytes_read);
        PageSize::parse_bytes([buf[16], buf[17]])
    }
    fn get_data(page: impl ValidPage) -> Vec<u8> {
        page.data_owned()
    }
}

fn main() {
    const SMALL_FILE_PATH: &str = "./data/small.sqlite3";
    const FLIGHTS_FILE_PATH: &str = "./data/flights-initial.db";
    let pager1 = Pager::start(SMALL_FILE_PATH);
    println!("{pager1:X?}");
    let pager2 = Pager::start(FLIGHTS_FILE_PATH);
    println!("{pager2:X?}");
}
