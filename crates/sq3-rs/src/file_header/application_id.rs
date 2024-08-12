use std::ops::Deref;

use sq3_derive::Name;
use sq3_parser::TypeName;

use crate::{result::SqliteResult, traits::ParseBytes};

/// # Application ID (4 Bytes)
///
///  The 4-byte big-endian integer at offset 68 is an "Application ID" that can
/// be set by the PRAGMA application_id command in order to identify the
/// database as belonging to or associated with a particular application. The
/// application ID is intended for database files used as an application
/// file-format. The application ID can be used by utilities such as file(1) to
/// determine the specific file type rather than just reporting
/// "Sqlite3 Database". A list of assigned application IDs can be seen by
/// consulting the magic.txt file in the Sqlite source repository.
#[derive(Debug, Default, Name)]
pub struct ApplicationId(u32);

impl Deref for ApplicationId {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ParseBytes for ApplicationId {
    const LENGTH_BYTES: usize = 4;
    fn parsing_handler(bytes: &[u8]) -> SqliteResult<Self> {
        let buf: [u8; Self::LENGTH_BYTES] = bytes.try_into()?;

        let value = u32::from_be_bytes(buf);

        Ok(Self(value))
    }
}
