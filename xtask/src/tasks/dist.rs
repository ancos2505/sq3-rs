use std::{env::Args, path::PathBuf};

use crate::{cli::XtaskArg, helpers::XtaskResult};

#[derive(Debug)]
pub(crate) struct Dist {
    /// Path to the dist folder
    pub(crate) path: PathBuf,
}
