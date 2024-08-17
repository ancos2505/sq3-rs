use std::fmt::Debug;
use std::io::SeekFrom;
use std::sync::{Arc, Mutex};
use std::{
    fs::{File, Metadata},
    io::{Cursor, Read, Seek},
};

use crate::result::SqliteResult;

#[allow(box_pointers)]
pub(super) struct SqliteIo {
    mode: SqliteIoMode,
    raw_io: Arc<Mutex<dyn SqliteRawIo>>,
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
        Ok(self
            .raw_io
            .lock()
            .or_else(|error| Err(std::io::Error::other(format!("SqliteIoMutex: {error}"))))?
            .read(buf)?)
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

        let mut file_metadata = Option::<Metadata>::None;
        let mode: SqliteIoMode;
        let raw_io: Arc<Mutex<dyn SqliteRawIo>> = match iter.next() {
            Some(path) => {
                if path.contains(":") {
                    mode = SqliteIoMode::InMemory;
                    Arc::new(Mutex::new(Cursor::new(Vec::<u8>::new())))
                } else {
                    let file = File::open(path)?;
                    mode = SqliteIoMode::File;
                    file_metadata = Some(file.metadata()?);
                    Arc::new(Mutex::new(file))
                }
            }
            None => {
                mode = SqliteIoMode::InMemory;
                Arc::new(Mutex::new(Cursor::new(Vec::<u8>::new())))
            }
        };
        let io = Self {
            mode,
            raw_io,
            file_metadata,
        };

        Ok(io)
    }
    pub fn rewind(&mut self) -> SqliteResult<()> {
        self.raw_io.lock()?.rewind()?;
        Ok(())
    }
    pub fn seek(&mut self, pos: u32) -> SqliteResult<()> {
        self.raw_io.lock()?.seek(SeekFrom::Start(pos.into()))?;
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
