use super::{constants::*, SyntaxError};

pub enum PollResponse {
    Next(Box<[u8]>),
    Pending,
    Done,
}

#[derive(Debug, Clone)]
pub struct Parser {
    // The target to iterate.
    path_to_array: Vec<String>,
    // The current path of current_index.
    current_path: Vec<String>,
    // The push/pop of nested structures.
    token_stack: Vec<(u8, usize)>,
    // The real index over the entire stream.
    current_index: usize,
    // The offset of the trim
    offset_index: usize,
    // The data stream.
    sink: Vec<u8>,
    // True, if in the array targeted by `path_to_array`.
    is_in_target_array: bool,
}

impl Parser {
    pub fn new(path_to_array: Vec<String>) -> Self {
        Parser {
            path_to_array,
            current_path: Vec::new(),
            token_stack: Vec::new(),
            current_index: 0,
            offset_index: 0,
            sink: Vec::new(),
            is_in_target_array: false,
        }
    }

    /// Append slice to the stream sink.
    pub fn push(&mut self, arr: &[u8]) {
        for item in arr.iter() {
            self.sink.push(*item);
        }
    }

    /// TODO Not sure anymore.
    pub fn poll(&mut self) -> PollResponse {
        PollResponse::Pending
    }

    /// Get the real index to the sink, accounting for trimming.
    fn index(&self) -> usize {
        self.current_index - self.offset_index
    }

    /// Trims stream to `[index()..]`.
    fn trim_left_sink(&mut self) {
        if self.sink.len() > self.index() {
            self.offset_index += self.current_index - self.offset_index;
            self.sink = self.sink.drain(self.index()..).collect();
        } else {
            self.offset_index = self.current_index;
            self.sink.clear();
        }
    }

    /// Push a char onto the `token_stack`.
    fn push_stack(&mut self, char: u8) {
        self.token_stack.push((char, self.current_index));
    }

    /// Remove the last char from the `token_stack`.
    fn pop_stack(&mut self) {
        self.token_stack.truncate(self.token_stack.len() - 1);
    }

    /// Advance to the next significant char, ignoring noise.
    fn next_char_event(&mut self) -> Option<u8> {
        loop {
            let c = *self.sink.get(self.index())?;
            match c {
                // Return significant chat.
                BRACKET_OPEN | BRACKET_CLOSE | COLON | COMMA | CURLY_BRACKET_OPEN
                | CURLY_BRACKET_CLOSE | DOUBLE_QUOTE => {
                    self.current_index += 1;
                    return Some(c);
                }
                BACK_SLASH | UTF8_2 => {
                    self.current_index += 2;
                }
                UTF8_3 => {
                    self.current_index += 3;
                }
                UTF8_4 => {
                    self.current_index += 4;
                }
                _ => {
                    // CARRIAGE_RETURN | NEWLINE | TAB | WHITE_SPACE | Etc
                    self.current_index += 1;
                }
            }
        }
    }

    fn parse_until_token(&mut self) -> Result<(), SyntaxError> {
        loop {
            match self.next_char_event() {
                Some(c) => match c {
                    COLON | COMMA | DOUBLE_QUOTE | BRACKET_OPEN | CURLY_BRACKET_OPEN => {
                        self.push_stack(c);
                        return Ok(());
                    }
                    // Maybe remove. Handled in handle_close()?
                    BRACKET_CLOSE | CURLY_BRACKET_CLOSE => {
                        self.pop_stack();
                        return Ok(());
                    }
                    _ => {
                        return Err(SyntaxError::new(
                            self.current_index - 1,
                            &format!("Invalid character '{}' found at {}.", c, self.current_index),
                        ));
                    }
                },
                None => {
                    return Ok(());
                }
            }
        }
    }

    fn match_close_event(&mut self) {
        match self.token_stack[..] {
            // End of array.
            [.., (BRACKET_OPEN, start_index), (BRACKET_CLOSE, end_index)] => {
                self.token_stack.truncate(self.token_stack.len() - 2)
            }
            // End of object.
            [.., (CURLY_BRACKET_OPEN, start_index), (CURLY_BRACKET_CLOSE, end_index)] => {
                self.token_stack.truncate(self.token_stack.len() - 2)
            }
            // End of key
            [.., (DOUBLE_QUOTE, start_index), (DOUBLE_QUOTE, end_index), (COLON, _)] => {
                // self.token_stack.truncate(self.token_stack.len() - 3)
            }
            // End of object value string
            [.., (COLON, _), (DOUBLE_QUOTE, start_index), (DOUBLE_QUOTE, end_index), (COMMA | CURLY_BRACKET_CLOSE, _)] => {
                self.token_stack.truncate(self.token_stack.len() - 6)
            }
            // End of object value others primitives
            [.., (COLON, start_index), (COMMA | CURLY_BRACKET_CLOSE, end_index)] => {
                self.token_stack.truncate(self.token_stack.len() - 3)
            }
            // End of array value
            [.., (start_token, start_index), (end_token, end_index)] => {
                // end_token: , ] }
            }
        }
        // TODO handle open and close events
    }
}
