use std::fmt::Debug;
use std::io::SeekFrom;
use std::{
    fs::{File, Metadata},
    io::{Cursor, Read, Seek},
};

use crate::result::SqliteResult;

pub(super) struct SqliteIo {
    mode: SqliteIoMode,
    raw_io: Box<dyn SqliteRawIo>,
    file_metadata: Option<Metadata>,
}

impl Debug for SqliteIo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SqliteIo")
            .field("mode", &self.mode)
            .field("raw_io", &"...")
            .finish()
    }
}

impl Read for SqliteIo {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.raw_io.read(buf)
    }
}

impl SqliteIo {
    pub(super) fn mode(&self) -> &SqliteIoMode {
        &self.mode
    }

    pub(super) fn file_metadata(&self) -> Option<&Metadata> {
        self.file_metadata.as_ref()
    }

    pub fn connect<S: AsRef<str>>(conn_str: S) -> SqliteResult<Self> {
        // TODO:  Implement database mode parameter.
        // TODO: Ex:
        // TODO:    - For in-memory: "sqlite://:memory:?mode=rwc"
        // TODO:    - For file.....: "sqlite://./data/small.sqlite3?mode=rwc"
        let s = conn_str.as_ref();
        let uri_str = if !s.contains("://") {
            format!("sqlite://{s}")
        } else {
            s.into()
        };
        let mut iter = uri_str.split("://");

        let proto = iter.next();
        // dbg!(&proto);
        let mut file_metadata = Option::<Metadata>::None;
        let mode: SqliteIoMode;
        let raw_io = match iter.next() {
            Some(path) => {
                // dbg!(&path);
                if path.contains(":") {
                    mode = SqliteIoMode::InMemory;
                    Box::new(Cursor::new(Vec::<u8>::new())) as Box<dyn SqliteRawIo>
                } else {
                    let file = File::open(path)?;
                    mode = SqliteIoMode::File;
                    file_metadata = Some(file.metadata()?);
                    Box::new(file) as Box<dyn SqliteRawIo>
                }
            }
            None => {
                mode = SqliteIoMode::InMemory;
                Box::new(Cursor::new(Vec::<u8>::new())) as Box<dyn SqliteRawIo>
            }
        };
        let io = Self {
            mode,
            raw_io,
            file_metadata,
        };
        // dbg!(&io);
        Ok(io)
    }
    pub fn rewind(&mut self) -> SqliteResult<()> {
        self.raw_io.rewind()?;
        Ok(())
    }
    pub fn seek(&mut self, pos: u32) -> SqliteResult<()> {
        // dbg!(&pos);
        self.raw_io.seek(SeekFrom::Start(pos.into()))?;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum SqliteIoMode {
    InMemory,
    File,
}

pub(crate) trait SqliteRawIo: Read + Send + Sync + Seek {}
impl SqliteRawIo for Cursor<Vec<u8>> {}
impl SqliteRawIo for File {}
