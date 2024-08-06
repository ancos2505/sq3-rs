mod build;
mod dist;
mod fuzzer;
mod help;

use crate::{
    cli::{Cli, Xtask},
    helpers::XtaskResult,
};

pub(crate) use self::{build::Build, dist::Dist, fuzzer::Fuzzer, help::Help};

pub(crate) struct Task;

impl Task {
    pub(crate) fn run(cli: Cli) -> XtaskResult<()> {
        let Cli { task, maybe_args } = cli;
        match task {
            Some(t) => t.run(maybe_args)?,
            None => Help.run(maybe_args)?,
        };

        Ok(())
    }
}
