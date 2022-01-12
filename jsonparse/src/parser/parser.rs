use std::str;

use super::{constants::*, SyntaxError};

pub enum PollResponse {
    Next(Box<[u8]>),
    Pending,
    Done,
}

#[derive(Debug, Clone)]
pub struct Parser {
    path_to_array: Option<Vec<String>>,
    state: ParseState,
    current_path: Vec<String>,
    depth_stack: Vec<u8>,
    current_index: usize,
    stream: Vec<u8>,
}

impl Parser {
    pub fn new(path_to_array: Option<Vec<String>>) -> Self {
        Parser {
            path_to_array,
            state: ParseState::AtStart,
            current_path: Vec::new(),
            depth_stack: Vec::new(),
            current_index: 0,
            stream: Vec::new(),
        }
    }

    pub fn push(&mut self, arr: &[u8]) {
        for item in arr.iter() {
            self.stream.push(*item);
        }
    }

    pub fn poll(&mut self) -> PollResponse {
        PollResponse::Pending
    }

    fn next_char(&mut self) -> Option<u8> {
        loop {
            let c = *self.stream.get(self.current_index)?;
            match c {
                BACK_SLASH | CARRIAGE_RETURN | NEWLINE | TAB | WHITE_SPACE => {
                    self.current_index += 1;
                    continue;
                }
                UTF8_2 => {
                    self.current_index += 2;
                    continue;
                }
                UTF8_3 => {
                    self.current_index += 3;
                    continue;
                }
                UTF8_4 => {
                    self.current_index += 4;
                    continue;
                }
                _ => {
                    self.current_index += 1;
                    return Some(c);
                }
            }
        }
    }

    /// Trims stream to `[current_index..]`.
    fn trim_left_stream(&mut self) {
        if self.stream.len() > self.current_index {
            self.stream = self.stream.drain(self.current_index..).collect();
        } else {
            self.stream.clear();
        }
    }

    fn at_start(&mut self) -> Result<(), SyntaxError> {
        match self.next_char() {
            Some(c) => match c {
                CURLY_BRACKET_OPEN => {
                    self.state = ParseState::PreKey;
                    Ok(())
                }
                BRACKET_OPEN => {
                    if let None = self.path_to_array {
                        self.state = ParseState::InStream;
                        Ok(())
                    } else {
                        Err(SyntaxError::new(
                            self.current_index - 1,
                            &format!("pathToArray provided for non-object."),
                        ))
                    }
                }
                _ => Err(SyntaxError::new(
                    self.current_index - 1,
                    &format!("Invalid character '{}' found at start.", c),
                )),
            },
            None => Ok(()),
        }
    }

    fn pre_key(&mut self) -> Result<(), SyntaxError> {
        match self.next_char() {
            Some(c) => match c {
                DOUBLE_QUOTE => {
                    self.state = ParseState::InKey;
                    Ok(())
                }
                _ => Err(SyntaxError::new(
                    self.current_index - 1,
                    &format!("Invalid character '{}' found at pre key. Expected (\").", c),
                )),
            },
            None => Ok(()),
        }
    }

    fn in_key(&mut self) -> Result<(), SyntaxError> {
        let start_index = self.current_index;
        loop {
            match self.next_char() {
                Some(c) => match c {
                    DOUBLE_QUOTE => {
                        // START: Nonsense
                        // The path needs to push/pop based on nested objects.
                        let end_index = self.current_index - 2;
                        let u8_slice = &self.stream[start_index..end_index];
                        let key = str::from_utf8(u8_slice).unwrap();
                        self.current_path.push(key.to_string());
                        // END: Nonsend
                        self.state = ParseState::PostKey;
                        return Ok(());
                    }
                    _ => {}
                },
                None => {
                    return Ok(());
                }
            }
        }
    }

    fn post_key(&mut self) -> Result<(), SyntaxError> {
        match self.next_char() {
            Some(c) => match c {
                COLON => {
                    self.state = ParseState::PreValue;
                    Ok(())
                }
                _ => Err(SyntaxError::new(
                    self.current_index - 1,
                    &format!(
                        "Invalid character '{}' found at after key. Expected (:).",
                        c
                    ),
                )),
            },
            None => Ok(()),
        }
    }

    fn pre_value(&mut self) -> Result<(), SyntaxError> {}

    fn in_value(&mut self) -> Result<(), SyntaxError> {
        let local_depth_index = self.depth_stack.len() - 1;
    }

    fn post_value(&mut self) -> Result<(), SyntaxError> {}

    fn pre_stream(&mut self) -> Result<(), SyntaxError> {}

    fn in_stream(&mut self) -> Result<(), SyntaxError> {}

    fn post_stream(&mut self) -> Result<(), SyntaxError> {}

    fn at_end(&mut self) -> Result<(), SyntaxError> {}
}
