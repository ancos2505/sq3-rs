use std::array::TryFromSliceError;
use std::error::Error as StdError;
use std::fmt::Debug;
use std::fmt::Display;
use std::io::Error as StdioError;
use std::num::{ParseFloatError, ParseIntError};
use std::sync::MutexGuard;
use std::sync::PoisonError;

use sq3_parser::Sq3ParserError;

use crate::io::SqliteRawIo;

pub type SqliteResult<T> = Result<T, SqliteError>;

#[derive(Debug)]
pub enum SqliteError {
    EmptyDb,
    InvalidFileUriMode,
    HeaderValidationError(String),
    TryFromSliceError(TryFromSliceError),
    ParseFloatError(ParseFloatError),
    ParseIntError(ParseIntError),
    StdioError(StdioError),
    Custom(String),
    ParsingField(FieldParsingError),
    InvalidPayloadSize(InvalidPayloadSizeError),
    SqlParser(Sq3ParserError),
    SqliteIoMutexPoisonError,
}

impl From<Sq3ParserError> for SqliteError {
    fn from(error: Sq3ParserError) -> Self {
        Self::SqlParser(error)
    }
}

impl From<PoisonError<MutexGuard<'_, (dyn SqliteRawIo + 'static)>>> for SqliteError {
    fn from(_: PoisonError<MutexGuard<'_, (dyn SqliteRawIo + 'static)>>) -> Self {
        Self::SqliteIoMutexPoisonError
    }
}
#[derive(Debug)]
pub struct FieldParsingError(pub String);

#[derive(Debug)]
pub struct InvalidPayloadSizeError(pub String);

impl Display for SqliteError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // TODO
        write!(f, "{:?}", self)
    }
}

impl From<TryFromSliceError> for SqliteError {
    fn from(error: TryFromSliceError) -> Self {
        Self::TryFromSliceError(error)
    }
}

impl From<ParseFloatError> for SqliteError {
    fn from(error: ParseFloatError) -> Self {
        Self::ParseFloatError(error)
    }
}

impl From<ParseIntError> for SqliteError {
    fn from(error: ParseIntError) -> Self {
        Self::ParseIntError(error)
    }
}

impl StdError for SqliteError {}

impl From<StdioError> for SqliteError {
    fn from(io_error: StdioError) -> Self {
        Self::StdioError(io_error)
    }
}
