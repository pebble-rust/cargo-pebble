// Copyright (c) 2019 RoccoDev
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use clap::ArgMatches;
use crate::commands::CommandResult;
use crate::errors::CommandError;

pub fn run(args: ArgMatches) -> CommandResult<CommandError> {
    let proj_name = args.value_of("name");

    if let Some(name) = proj_name {
        Ok(())
    }
    else {
        Err(CommandError {error_text: "No 'name' argument was provided.".to_owned()})
    }
}