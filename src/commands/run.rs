// Copyright (c) 2019 RoccoDev
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use clap::ArgMatches;
use crate::commands::CommandResult;
use crate::errors::CommandError;
use crate::util::build_script::{run_build, print_stderr};
use std::process::{Command, Output};

pub fn run(args: &ArgMatches) -> CommandResult<CommandError> {
    let (_, args) = args.subcommand();
    let args = args.unwrap();

    let emulator_type = args.value_of("emulator");
    let emulator_type = match emulator_type {
        Some(t) => t,
        _ => "diorite"
    };

    let logs = if args.is_present("logs") { "--logs" } else { "" };

    run_build()?;
    run_cmd("pebble", &["install", format!("--emulator={}", emulator_type).as_str(), logs])
}

pub fn run_cmd<'a>(cmd: &'a str, args: &[&str]) -> CommandResult<CommandError> {
    let mut command = Command::new(cmd);
    let command = command.args(args);
    let mut child = command.spawn().expect("Process spawn failed");
    child.wait().expect("Child process was interrupted.");
    Ok(())
}
