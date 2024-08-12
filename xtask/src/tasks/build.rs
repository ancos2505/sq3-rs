use sq3_derive::Name;
use sq3_parser::TypeName;
use std::{
    env::Args,
    process::{Command, Output, Stdio},
    str::FromStr,
};

use crate::{
    cli::Xtask,
    helpers::{XtaskError, XtaskResult},
};

#[derive(Debug, Default, Name)]
pub(crate) struct Build;

#[derive(Debug, Default)]
pub(self) struct XtaskArgs {
    // #[arg(long, short)]
    /// Enable release
    verbose: bool,
    release: bool,
}
impl XtaskArgs {
    fn to_vec(&self) -> Vec<&'static str> {
        let mut goal = vec!["build"];
        if self.release {
            goal.push("--release")
        };
        goal
    }
}

impl FromStr for Build {
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
impl TryFrom<Option<Args>> for XtaskArgs {
    type Error = XtaskError;

    fn try_from(maybe_values: Option<Args>) -> Result<Self, Self::Error> {
        if let Some(mut args) = maybe_values {
            let mut xtask_args = XtaskArgs::default();
            while let Some(arg) = args.next() {
                match &*arg {
                    "--verbose" | "-v" => xtask_args.verbose = true,
                    "--release" | "-r" => xtask_args.release = true,
                    _ => (),
                };
            }
            Ok(xtask_args)
        } else {
            Ok(Default::default())
        }
    }
}

impl Xtask for Build {
    fn run(&self, maybe_args: Option<Args>) -> XtaskResult<()> {
        let args = XtaskArgs::try_from(maybe_args)?;

        let output = Command::new("cargo")
            .args(&args.to_vec())
            // .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            // .stderr(Stdio::null())
            .output()?;

        // let child_stdin = child.stdin.as_mut().unwrap();

        // child_stdin.write_all(data.as_bytes())?;
        // Close stdin to finish and avoid indefinite blocking
        // drop(child_stdin);

        let Output { stdout, stderr, .. } = output;

        // let stdout = String::from_utf8_lossy(&output.stdout);

        let stdout = String::from_utf8(stdout)?;
        let stderr = String::from_utf8(stderr)?;

        println!("{}", stdout);

        if args.verbose {
            eprintln!("{}", stderr);
        }

        Ok(())
    }
}
