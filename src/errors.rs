// Copyright (c) 2019 RoccoDev
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone)]
pub struct CommandError {
    pub error_text: String
}

impl Display for CommandError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Command error: {}", self.error_text)
    }
}
