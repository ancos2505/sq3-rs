use std::fmt::Debug;

use sq3_derive::Name;

use crate::{
    field_parsing_error,
    result::SqliteResult,
    traits::{ParseBytes, TypeName},
};

/// Reserved for expansion. Must be zero. (20 Bytes)
#[derive(Default, Name)]
pub struct ReservedForExpansion([u8; 20]);

impl Debug for ReservedForExpansion {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple(Self::NAME).finish()
    }
}

impl ParseBytes for ReservedForExpansion {
    const LENGTH_BYTES: usize = 20;

    fn parsing_handler(bytes: &[u8]) -> SqliteResult<Self> {
        for byte in bytes.iter() {
            if *byte != b'\0' {
                return Err(field_parsing_error! {Self::NAME.into()});
            }
        }
        Ok(Default::default())
    }
}
