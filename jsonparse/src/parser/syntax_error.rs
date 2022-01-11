use std::{error::Error, fmt};

#[derive(Debug)]
pub struct SyntaxError {
    position: usize,
    message: String,
}

impl SyntaxError {
    pub fn new(position: usize, message: &str) -> Self {
        SyntaxError {
            position,
            message: message.to_string(),
        }
    }
}

impl Error for SyntaxError {}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "JSON Syntax Error at {}; {}",
            self.position, self.message
        )
    }
}
