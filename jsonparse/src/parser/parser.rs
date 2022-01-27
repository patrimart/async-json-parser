use std::ops::Index;

use super::{constants::*, SyntaxError};

pub enum PollResponse {
    Next(Box<[u8]>),
    Pending,
    Done,
}

#[derive(Debug, Clone)]
pub struct Parser {
    path_to_array: Vec<String>,
    current_path: Vec<String>,
    token_stack: Vec<(u8, usize)>,
    current_index: usize,
    stream: Vec<u8>,
    is_in_target_array: bool,
}

impl Parser {
    pub fn new(path_to_array: Vec<String>) -> Self {
        Parser {
            path_to_array,
            current_path: Vec::new(),
            token_stack: Vec::new(),
            current_index: 0,
            stream: Vec::new(),
            is_in_target_array: false,
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

    fn push_stack(&mut self, char: u8) {
        self.token_stack.push((char, self.current_index));
    }

    fn pop_stack(&mut self) {
        self.token_stack.pop();
    }

    fn parse_until_token(&mut self) -> Result<(), SyntaxError> {
        loop {
            match self.next_char() {
                Some(c) => match c {
                    COLON => {
                        // End of property
                    }
                    COMMA => {
                        // End of array item
                    }
                    DOUBLE_QUOTE => {
                        self.push_stack(c);
                    }
                    BRACKET_OPEN | CURLY_BRACKET_OPEN => {
                        self.push_stack(c);
                    }
                    BRACKET_CLOSE | CURLY_BRACKET_CLOSE => {
                        self.pop_stack();
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

    fn handle_close(&mut self) {
        match self.token_stack[..] {
            // End of array.
            [.., (BRACKET_OPEN, start_index), (BRACKET_CLOSE, end_index), (end_token, _)] => {
                // end_token: , ] }
            }
            [.., (CURLY_BRACKET_OPEN, start_index), (CURLY_BRACKET_CLOSE, end_index), (end_token, _)] =>
            // End of object.
            {
                // end_token: , ] } EOF
            }
            // End of true
            [.., (b't', start_index), (b'r', _), (b'u', _), (b'e', end_index), (end_token, _)] => {
                // end_token: , ] }
            }
            [.., (b'f', start_index), (b'a', _), (b'l', _), (b's', _), (b'e', end_index), (end_token, _)] =>
            // End of false
            {
                // end_token: , ] }
            }
            // End of null
            [.., (b'n', start_index), (b'u', _), (b'l', _), (b'l', end_index), (end_token, _)] => {
                // end_token: , ] }
            }
            // End of number
            [.., (prim_open, start_index), (prim_close, end_index), (end_token, _)] => {
                // end_token: , ] }
            } // [.., (DOUBLE_QUOTE, start_index), (DOUBLE_QUOTE, end_index), (COLON, index)] => {
              //     // end of object key is [start_index, end_index]
              // }
              // // [.., (BRACKET_CLOSE, index)] => {}
              // [.., (BRACKET_CLOSE, index)] => {
              //     // End of array
              // }
              // // [.., (CURLY_BRACKET_CLOSE, index)] => {}
              // [.., (CURLY_BRACKET_CLOSE, index)] => {
              //     // End of object
              // }
        }
        // TODO handle open and close events
    }

    // fn at_start(&mut self) -> Result<(), SyntaxError> {
    //     match self.next_char() {
    //         Some(c) => match c {
    //             CURLY_BRACKET_OPEN => {
    //                 self.state = ParseState::PreKey;
    //                 Ok(())
    //             }
    //             BRACKET_OPEN => {
    //                 if let None = self.path_to_array {
    //                     self.is_in_stream = true;
    //                     Ok(())
    //                 } else {
    //                     Err(SyntaxError::new(
    //                         self.current_index - 1,
    //                         &format!("pathToArray provided for non-object."),
    //                     ))
    //                 }
    //             }
    //             _ => Err(SyntaxError::new(
    //                 self.current_index - 1,
    //                 &format!("Invalid character '{}' found at start.", c),
    //             )),
    //         },
    //         None => Ok(()),
    //     }
    // }

    // fn pre_key(&mut self) -> Result<(), SyntaxError> {
    //     match self.next_char() {
    //         Some(c) => match c {
    //             DOUBLE_QUOTE => {
    //                 self.state = ParseState::InKey;
    //                 Ok(())
    //             }
    //             _ => Err(SyntaxError::new(
    //                 self.current_index - 1,
    //                 &format!("Invalid character '{}' found at pre key. Expected (\").", c),
    //             )),
    //         },
    //         None => Ok(()),
    //     }
    // }

    // fn in_key(&mut self) -> Result<(), SyntaxError> {
    //     let start_index = self.current_index;
    //     loop {
    //         match self.next_char() {
    //             Some(c) => match c {
    //                 DOUBLE_QUOTE => {
    //                     // START: Nonsense
    //                     // The path needs to push/pop based on nested objects.
    //                     let end_index = self.current_index - 2;
    //                     let u8_slice = &self.stream[start_index..end_index];
    //                     let key = str::from_utf8(u8_slice).unwrap();
    //                     self.current_path.push(key.to_string());
    //                     // END: Nonsend
    //                     self.state = ParseState::PostKey;
    //                     return Ok(());
    //                 }
    //                 _ => {}
    //             },
    //             None => {
    //                 return Ok(());
    //             }
    //         }
    //     }
    // }

    // fn post_key(&mut self) -> Result<(), SyntaxError> {
    //     match self.next_char() {
    //         Some(c) => match c {
    //             COLON => {
    //                 self.state = ParseState::PreValue;
    //                 Ok(())
    //             }
    //             _ => Err(SyntaxError::new(
    //                 self.current_index - 1,
    //                 &format!("Invalid character '{c}' found at after key. Expected (:).",),
    //             )),
    //         },
    //         None => Ok(()),
    //     }
    // }

    // fn pre_value(&mut self) -> Result<(&[u8]), SyntaxError> {
    //     match self.next_char() {
    //         Some(c) => match c {
    //             BRACKET_OPEN => {
    //                 self.state = ParseState::InValue;
    //                 Ok(())
    //             }
    //             CURLY_BRACKET_OPEN => {
    //                 self.state = ParseState::InValue;
    //                 Ok(())
    //             }
    //             DOUBLE_QUOTE => {
    //                 self.state = ParseState::InValue;
    //                 Ok(())
    //             }
    //             c if PRIMITIVE_OPEN.contains(&c) => {
    //                 self.state = ParseState::InValue;
    //                 let start_index = self.current_index;
    //                 match c {
    //                     b't' => {
    //                         self.current_index += 3;
    //                     }
    //                     b'f' => {
    //                         self.current_index += 4;
    //                     }
    //                     b'n' => {
    //                         self.current_index += 3;
    //                     }
    //                     n => loop {},
    //                 }
    //                 Ok(())
    //             }
    //             _ => Err(SyntaxError::new(
    //                 self.current_index - 1,
    //                 &format!(
    //                     "Invalid character '{}' found at pre value. Expected (\").",
    //                     c
    //                 ),
    //             )),
    //         },
    //         None => Ok(()),
    //     }
    // }

    // fn in_value(&mut self) -> Result<(), SyntaxError> {
    //     let local_depth_index = self.token_stack.len() - 1;
    // }

    // fn post_value(&mut self) -> Result<(), SyntaxError> {}

    // fn at_end(&mut self) -> Result<(), SyntaxError> {}
}
