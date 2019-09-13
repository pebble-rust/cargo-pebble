// Copyright (c) 2019 RoccoDev
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::path::{Path, PathBuf};
use std::fs;
use crate::commands::CommandResult;
use crate::errors::CommandError;
use serde_derive::Deserialize;

#[derive(Deserialize)]
struct CargoToml {
    package: Package
}

#[derive(Deserialize)]
struct Package {
    name: String
}

pub fn get_crate_name(path: &PathBuf) -> Result<String, CommandError> {
    let path = path.join(Path::new("Cargo.toml"));
    let contents = fs::read_to_string(path);
    if contents.is_err() {
        return Err(CommandError {error_text: "Cargo.toml was not found in this directory.".to_owned()});
    }
    let contents = contents.unwrap();
    let toml = toml::from_str(contents.as_str());
    if toml.is_err() {
        return Err(CommandError {error_text: "Invalid Cargo.toml".to_owned()});
    }
    let toml: CargoToml = toml.unwrap();
    Ok(toml.package.name.replace("-", "_"))
}