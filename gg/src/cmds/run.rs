use std::env;
use std::ops::Add;
use shlex;
use clap::Args;
use std::os::unix::process::CommandExt;
use std::process::Command;
use gg_config::OptionalLoadedConfig;
use crate::result::{error, Result};
use gg_tui::{ep_warning};

#[derive(Args)]
#[command(about = "run a pre-defined script")]
pub(crate) struct RunCommand {
    #[arg(trailing_var_arg = true)]
    script_and_args: Vec<String>,
}

impl RunCommand {
    pub(crate) async fn run(&self) -> Result<()> {
        if self.script_and_args.is_empty() {
            return Err(error("no script specified"));
        }

        let config = gg_config::auto_load_for_repo(env::current_dir().unwrap()).await;
        let config = match config {
            Ok(config) => config.get(),
            Err(err) => {
                return Err(error(&format!("failed to load config: {}", err)));
            }
        };

        let script = self.script_and_args[0].to_string();
        let args: Vec<&str> = self.script_and_args[1..].iter().map(|s| s.as_str()).collect();

        let script_config = match config.scripts.get(&script) {
            Some(c) => c,
            None => {
                return Err(error(&format!("script '{}' not found", script)));
            }
        };

        let root = match gg_git::get_root().await {
            Ok(root) => root,
            Err(_) => {
                ep_warning!("failed to get git root, run script in current directory");
                env::current_dir().unwrap()
            }
        };

        let joined_args = shlex::try_join(args).unwrap();
        let command_and_args = script_config.command.to_string().add(" ").add(&joined_args);

        println!("> {}", command_and_args);

        let args = shlex::split(&command_and_args).unwrap();
        let program = args[0].clone();
        let args = &args[1..];

        let err = Command::new(program).args(args).current_dir(root).exec();
        return Err(error(&format!("failed to execute command: {}", err)));
    }
}