use std::error::Error as StdError;
use std::io::Error as StdioError;
use std::path::{Path, PathBuf};
use std::string::FromUtf8Error;

pub(crate) type XtaskResult<T> = Result<T, XtaskError>;

#[derive(Debug)]
pub(crate) struct XtaskError(ErrorContent);

impl XtaskError {
    pub(crate) fn msg(msg: impl AsRef<str>) -> Self {
        Self(ErrorContent(msg.as_ref().into()))
    }
}
impl From<StdioError> for XtaskError {
    fn from(value: StdioError) -> Self {
        Self(ErrorContent(value.to_string()))
    }
}

impl From<Box<dyn StdError>> for XtaskError {
    fn from(value: Box<dyn StdError>) -> Self {
        Self(ErrorContent(value.to_string()))
    }
}

impl From<FromUtf8Error> for XtaskError {
    fn from(value: FromUtf8Error) -> Self {
        Self(ErrorContent(value.to_string()))
    }
}

#[derive(Debug)]
struct ErrorContent(String);

impl From<&str> for ErrorContent {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}

impl From<String> for ErrorContent {
    fn from(value: String) -> Self {
        Self(value)
    }
}

pub(crate) fn project_root() -> XtaskResult<PathBuf> {
    let project_root = Path::new(
        &std::env::var("CARGO_MANIFEST_DIR")
            .unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_owned()),
    )
    .ancestors()
    .nth(1)
    .ok_or(XtaskError::msg("Error on getting project root path"))?
    .to_path_buf();

    Ok(project_root)
}
