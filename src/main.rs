// Copyright (c) 2019 RoccoDev
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

#[macro_use]
extern crate clap;

use clap::App;

fn main() {
    let yml = load_yaml!("../res/cli.yml");
    let matches = App::from_yaml(yml).get_matches();
}
