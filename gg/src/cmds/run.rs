use clap::Args;
use crate::result::{error, Result};

#[derive(Args)]
#[command(about = "run a pre-defined script")]
pub(crate) struct RunCommand {
    #[arg(trailing_var_arg = true)]
    script_and_args: Vec<String>,
}

impl RunCommand {
    pub(crate) fn run(&self) -> Result<()> {
        if self.script_and_args.is_empty() {
            return Err(error("no script specified"));
        }

        println!("run script: {}", self.script_and_args[0]);
        println!("args: {:?}", &self.script_and_args[1..]);
        return Ok(());
    }
}