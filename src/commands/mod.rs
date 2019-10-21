// Copyright (c) 2019 RoccoDev
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

pub mod new;
pub mod build;
pub mod run;

pub type CommandResult<T> = Result<(), T>;