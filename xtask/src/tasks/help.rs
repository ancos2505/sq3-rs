use std::{env::Args, str::FromStr};

use sq3_derive::Name;
use sq3_rs::TypeName;

use crate::{
    cli::Xtask,
    helpers::{XtaskError, XtaskResult},
};

#[derive(Debug, Default, Name)]
pub(crate) struct Help;

impl Xtask for Help {
    fn run(&self, maybe_args: Option<Args>) -> XtaskResult<()> {
        println!("{}", usage_str());
        Ok(())
    }
}

impl FromStr for Help {
    type Err = XtaskError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == Self::NAME.to_lowercase() {
            Ok(Default::default())
        } else {
            Err(XtaskError::msg(format!(
                "Not a valid input for parsing `{}`",
                Self::NAME
            )))
        }
    }
}

fn usage_str() -> &'static str {
    r#"Available tasks:
    build  # Build the Application
    help   # This Task
    fuzzer # Run the Fuzzing tests
"#

//     r#"Available tasks:
//     build # Build the Application
//     clean # Clean temporary files for building
//     help  # This Task
//     run   # Run the Application
// "#

}
