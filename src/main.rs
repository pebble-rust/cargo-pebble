// Copyright (c) 2019 RoccoDev
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

#[macro_use]
extern crate clap;
extern crate fs_extra;
extern crate toml;
#[macro_use]
extern crate serde_derive;

mod commands;
pub mod errors;
pub mod util;

use clap::{App, AppSettings};
use std::panic;
use crate::commands::CommandResult;
use crate::errors::{CommandError};
use std::fmt::Display;

fn main() {
    let yml = load_yaml!("../res/cli.yml");
    let app = App::from_yaml(yml)
        .setting(AppSettings::ArgRequiredElseHelp);
    let matches = app.get_matches();
    let matches = matches.subcommand();
    let (_, matches) = matches;
    let matches = matches.unwrap();

    let result: CommandResult<CommandError> = match matches.subcommand_name() {
        Some("new") => commands::new::run(matches),
        Some("build") => commands::build::run(matches),
        Some("run") => commands::run::run(matches),
        _ => Ok(())
    };
    if result.is_err() {
        eprintln!("{}", result.err().unwrap());
    }
}
