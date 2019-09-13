// Copyright (c) 2019 RoccoDev
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use clap::ArgMatches;
use std::process::{Command, Output};
use std::error::Error;
use crate::commands::CommandResult;
use crate::errors::{CommandError};
use crate::util::build_script::run_build;

pub fn run(args: ArgMatches) -> CommandResult<CommandError> {
    run_build()
}