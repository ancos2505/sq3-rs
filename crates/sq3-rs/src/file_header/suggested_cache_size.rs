use std::ops::Deref;

use sq3_derive::Name;
use sq3_parser::TypeName;

use crate::{result::SqliteResult, traits::ParseBytes};

/// # Suggested cache size (4 Bytes)
///
///  The 4-byte big-endian signed integer at offset 48 is the suggested cache
/// size in pages for the database file. The value is a suggestion only and
/// Sqlite is under no obligation to honor it. The absolute value of the integer
/// is used as the suggested size. The suggested cache size can be set using the
/// default_cache_size pragma.
#[derive(Debug, Default, Name)]
pub struct SuggestedCacheSize(u32);
impl Deref for SuggestedCacheSize {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ParseBytes for SuggestedCacheSize {
    const LENGTH_BYTES: usize = 4;

    fn parsing_handler(bytes: &[u8]) -> SqliteResult<Self> {
        let buf: [u8; Self::LENGTH_BYTES] = bytes.try_into()?;

        let database_size = u32::from_be_bytes(buf);

        Ok(Self(database_size))
    }
}
