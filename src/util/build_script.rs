// Copyright (c) 2019 RoccoDev
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::process::{Command, Output};
use crate::commands::CommandResult;
use crate::errors::CommandError;
use std::fs::File;
use std::path::Path;
use std::time::SystemTime;
use crate::util::cargo_lookup::get_crate_name;

/// Runs the build script.
/// Cargo doesn't include other files in the build, so we have to write a Rust implementation.
/// The Bash source is in res/pebble_build.sh
pub fn run_build() -> CommandResult<CommandError> {
    let crate_path = std::env::current_dir().unwrap();
    let crate_name = get_crate_name(&crate_path);
    if crate_name.is_err() {
        return Err(crate_name.err().unwrap());
    }
    let crate_name = crate_name.unwrap();
    println!("Building crate {}", crate_name);

    let start = SystemTime::now();

    println!("Cleaning up workspace...");
    run_cmd("pebble", &["clean"])?;
    run_cmd("cargo", &["clean"])?;

    println!("Building Rust crate...");
    run_cmd("cargo", &["+nightly", "build", "--release", "--target", "thumbv7m-none-eabi"])?;

    let deps_dir = crate_path.join(Path::new("target/thumbv7m-none-eabi/release/deps"));
    if !deps_dir.exists() || !deps_dir.is_dir() {
        return Err(CommandError {error_text: "Deps files were not found for target 'thumbv7m-none-eabi'. \
        Are you on the right target? Did the build fail?".to_owned()});
    }

    let crates_dir = deps_dir.parent().unwrap().join(Path::new("crates"));
    if !crates_dir.exists() {
        match std::fs::create_dir(&crates_dir) {
            Err(e) => return Err(CommandError {error_text: "Could not create crates dir.".to_owned()}),
            _ => {}
        }
    }

    let mut move_paths = Vec::new();
    for file in deps_dir.read_dir().unwrap() {
        let path = file.unwrap().path();
        if path.is_file() {
            match path.extension() {
                Some(ext) => {
                    if ext == "rlib" {
                        move_paths.push(path);
                    }
                },
                _ => {}
            }
        }
    }
    fs_extra::move_items(&move_paths, crates_dir.to_str().unwrap(),
                         &fs_extra::dir::CopyOptions::new());

    let deps_path = crate_path.join(Path::new("target/release/deps"));
    let deps_path = deps_path.to_str().unwrap();

    println!("Preparing Pebble build...");
    std::env::set_current_dir(&deps_dir);
    run_cmd("sh", &["-c", "ar x *.a"])?;
    run_cmd("sh", &["-c", "find . -type f ! -name '*.rcgu.o' -delete"])?;
    run_cmd("sh", &["-c", format!("find . -type f -name '{}*.o' -delete", crate_name).as_str()])?;
    std::env::set_current_dir(crate_path.to_str().unwrap());

    let rustc_dir = crate_path.join(Path::new("rustc_temp"));
    match std::fs::create_dir(&rustc_dir) {
        Err(e) => return Err(CommandError {error_text: "Could not create rustc temp dir.".to_owned()}),
        _ => {}
    }
    let rustc_str = rustc_dir.to_str().unwrap();

    println!("Finalizing Rust build...");
    run_cmd("rustc", &["+nightly", "-L", crates_dir.to_str().unwrap(), "-L", deps_path,
        "--color","always","--emit=llvm-ir","--crate-type=staticlib",
        "--target","thumbv7m-none-eabi",format!("{}/src/lib.rs", crate_path.to_str().unwrap()).as_str(),
        "-A","dead-code","-o", format!("{}/lib.ll", rustc_str).as_str()])?;

    run_cmd("llc", &["-mtriple=thumbv7m-none-eabi","-relocation-model=pic","-march=thumb","-mattr=+thumb2",
        "-mattr=+soft-float","-mcpu=cortex-m3","--asm-verbose=false","-o","rustc_temp/lib.s","rustc_temp/lib.ll"])?;
    run_cmd("arm-none-eabi-as", &["-c","rustc_temp/lib.s","-o","rustc_temp/lib.o"])?;
    run_cmd("sh", &["-c", "mv rustc_temp/*.o target/thumbv7m-none-eabi/release/deps"])?;

    println!("Building Pebble package...");
    run_cmd("pebble", &["build"])?;

    run_cmd("cargo", &["clean"])?;
    run_cmd("rm", &["-r", "rustc_temp"])?;

    println!("BUILD SUCCESS after {} seconds.", start.elapsed().unwrap().as_secs());

    Ok(())
}

fn run_cmd<'a>(cmd: &'a str, args: &[&str]) -> CommandResult<CommandError> {
    let mut command = Command::new(cmd);
    let command = command.args(args);
    let output = command.output().unwrap();

    if !output.status.success() {
        let code = output.status.code().unwrap();
        print_stderr(output);
        return Err(CommandError {error_text: format!("Build process failed. (exit code {}, command: '{}')",
                                              code, cmd)});
    }
    Ok(())
}

fn print_stderr(output: Output) {
    String::from_utf8(output.stderr).unwrap()
        .lines()
        .for_each(|l| println!("{}", l));
}