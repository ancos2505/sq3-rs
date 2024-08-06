use std::{env::Args, fmt::Debug};

use crate::{
    helpers::{project_root, XtaskError, XtaskResult},
    tasks::{Build, Dist, Fuzzer, Help},
};

pub(crate) trait Xtask: Debug {
    fn run(&self, maybe_args: Option<Args>) -> XtaskResult<()>;
}

pub(crate) trait XtaskArg: Debug {}

/// cargo xtask
#[derive(Debug, Default)]
// #[command(author, version, about)]
pub(crate) struct Cli {
    // #[command(subcommand)]
    pub(crate) task: Option<Box<dyn Xtask>>,
    pub(crate) maybe_args: Option<Args>,
}

const CLI_NUM_FIELDS: usize = 2;

impl Cli {
    pub(crate) fn parse() -> XtaskResult<Self> {
        let mut args = std::env::args();

        let _ = args.next();
        let maybe_task_str = args.next();

        if let Some(task_to_parse) = maybe_task_str {
            let task = task_to_parse
                .parse::<Help>()
                .ok()
                .map(|task| Box::new(task) as Box<dyn Xtask>)
                .or_else(|| {
                    task_to_parse
                        .parse::<Build>()
                        .ok()
                        .map(|task| Box::new(task) as Box<dyn Xtask>)
                })
                .or_else(|| {
                    task_to_parse
                        .parse::<Fuzzer>()
                        .ok()
                        .map(|task| Box::new(task) as Box<dyn Xtask>)
                });

            Ok(Self {
                task,
                maybe_args: Some(args),
            })
        } else {
            Ok(Default::default())
        }
    }
}
