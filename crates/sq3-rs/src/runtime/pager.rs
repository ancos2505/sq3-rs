mod page;

use std::{io::Read, num::NonZeroU32};

use page::ValidPage;

use crate::{
    file_header::{PageSize, SqliteHeader},
    io::SqliteIo,
    result::SqliteResult,
    traits::ParseBytes,
};

use self::page::Page;

#[derive(Debug)]
pub(crate) struct Pager {
    io: SqliteIo,
    file_header: SqliteHeader,
    current_page: Option<Vec<u8>>,
}

impl Pager {
    pub fn start<S: AsRef<str>>(conn_str: S) -> SqliteResult<Self> {
        let mut io = SqliteIo::connect(conn_str)?;
        let file_header = Self::get_file_header(&mut io)?;

        Ok(Self {
            current_page: None,
            io,
            file_header,
        })
    }
    pub fn get_page(&mut self, pos: NonZeroU32) -> SqliteResult<Vec<u8>> {
        self.io.rewind()?;
        let offset_position = (pos.get() - 1) * u32::from(self.file_header.page_size());
        self.io.seek(offset_position)?;
        let data = match self.file_header.page_size() {
            PageSize::L512 => {
                const PAGE_SIZE: usize = 512;
                let mut buf: [u8; PAGE_SIZE] = [0; PAGE_SIZE];
                let bytes_read = self.io.read(&mut buf)?;

                let page = Page::<PAGE_SIZE>::parse(pos.into(), buf);
                Self::get_data(page)
            }
            PageSize::L1024 => {
                const PAGE_SIZE: usize = 1024;
                let mut buf: [u8; PAGE_SIZE] = [0; PAGE_SIZE];
                let bytes_read = self.io.read(&mut buf)?;

                let page = Page::<PAGE_SIZE>::parse(pos.into(), buf);
                Self::get_data(page)
            }
            PageSize::L2048 => {
                const PAGE_SIZE: usize = 2048;
                let mut buf: [u8; PAGE_SIZE] = [0; PAGE_SIZE];
                let bytes_read = self.io.read(&mut buf)?;

                let page = Page::<PAGE_SIZE>::parse(pos.into(), buf);
                Self::get_data(page)
            }
            PageSize::L4096 => {
                const PAGE_SIZE: usize = 4096;
                let mut buf: [u8; PAGE_SIZE] = [0; PAGE_SIZE];
                let bytes_read = self.io.read(&mut buf)?;

                let page = Page::<PAGE_SIZE>::parse(pos.into(), buf);
                Self::get_data(page)
            }
            PageSize::L8192 => {
                const PAGE_SIZE: usize = 8192;
                let mut buf: [u8; PAGE_SIZE] = [0; PAGE_SIZE];
                let bytes_read = self.io.read(&mut buf)?;

                let page = Page::<PAGE_SIZE>::parse(pos.into(), buf);
                Self::get_data(page)
            }
            PageSize::L16384 => {
                const PAGE_SIZE: usize = 16384;
                let mut buf: [u8; PAGE_SIZE] = [0; PAGE_SIZE];
                let bytes_read = self.io.read(&mut buf)?;

                let page = Page::<PAGE_SIZE>::parse(pos.into(), buf);
                Self::get_data(page)
            }
            PageSize::L32768 => {
                const PAGE_SIZE: usize = 32768;
                let mut buf: [u8; PAGE_SIZE] = [0; PAGE_SIZE];
                let bytes_read = self.io.read(&mut buf)?;

                let page = Page::<PAGE_SIZE>::parse(pos.into(), buf);
                Self::get_data(page)
            }
            PageSize::L65536 => {
                const PAGE_SIZE: usize = 65536;
                let mut buf: [u8; PAGE_SIZE] = [0; PAGE_SIZE];
                let bytes_read = self.io.read(&mut buf)?;

                let page = Page::<PAGE_SIZE>::parse(pos.into(), buf);
                Self::get_data(page)
            }
        };
        Ok(data)
    }

    fn get_file_header(io: &mut impl Read) -> SqliteResult<SqliteHeader> {
        let mut buf = [0; SqliteHeader::LENGTH_BYTES];
        let bytes_read = io.read(&mut buf)?;

        Ok(SqliteHeader::parse_bytes(&buf)?)
    }

    fn get_data(page: impl ValidPage) -> Vec<u8> {
        page.data_owned()
    }
}
