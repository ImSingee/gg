use clap::{ArgMatches, Args, Command, command, FromArgMatches};
use cmds::RunCommand;
use result::Result;
use crate::result::{exit};

mod cmds;
mod result;

pub fn get_cmd() -> Command {
    command!()
        .allow_external_subcommands(true)
        .subcommand(
            RunCommand::augment_args(Command::new("run"))
        )
}

pub fn run(mut cmd: Command, matches: ArgMatches) {
    let result: Result<()> = match matches.subcommand() {
        Some(("run", m)) => {
            RunCommand::from_arg_matches(m).map_err(|err| err.exit()).unwrap().run()
        }
        _ => {
            cmd.print_long_help().expect("cannot print help message");
            Ok(())
        }
    };

    exit(result);
}

/// like cmd.get_matches, but will try to run subcommand if there is unknown argument
pub fn get_matches(cmd: &mut Command) -> ArgMatches {
    let matches = cmd.get_matches_mut();


    if let Some((subcommand, _)) = matches.subcommand() {
        if !cmd.get_subcommands().any(|c| c.get_name() == subcommand) {
            // unknown subcommand
            // TODO should be three case:
            // 1. scripts
            // 2. extension
            // 3. custom tool
            // or unknown

            // TODO we only use case 1 now
            let action = "run";

            let original_args: Vec<String> = std::env::args().collect();

            let mut new_args = Vec::with_capacity(original_args.len() + 1);
            new_args.push(original_args[0].clone());
            new_args.push(action.to_string());
            new_args.extend_from_slice(&original_args[1..]);

            return cmd.try_get_matches_from_mut(new_args).map_err(|e| e.exit()).unwrap();
        }
    }

    matches
}
