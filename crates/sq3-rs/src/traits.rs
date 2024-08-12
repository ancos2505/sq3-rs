use crate::result::{InvalidPayloadSizeError, SqliteError, SqliteResult};

use sq3_parser::TypeName;

pub(super) trait ParseBytes
where
    Self: Sized + TypeName,
{
    const LENGTH_BYTES: usize;

    fn parsing_handler(bytes: &[u8]) -> SqliteResult<Self>;

    fn check_payload_size(bytes: &[u8]) -> SqliteResult<()> {
        if bytes.len() < Self::LENGTH_BYTES {
            Err(SqliteError::InvalidPayloadSize(InvalidPayloadSizeError(
                format!("Invalid input size for {}", Self::NAME),
            )))
        } else {
            Ok(())
        }
    }
    fn parse_bytes(bytes: &[u8]) -> SqliteResult<Self> {
        Self::check_payload_size(bytes)?;
        Self::parsing_handler(bytes)
    }
}

pub(crate) trait ValidateParsed
where
    Self: Sized + ParseBytes,
{
    fn validate_parsed(&self) -> SqliteResult<()>;
}
