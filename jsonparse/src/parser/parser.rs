use super::constants::*;

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

    fn next_char(&self) -> Option<u8> {
        loop {
            let c = *self.stream.get(self.current_index)?;
            match c {
                BACK_SLASH | CARRIAGE_RETURN | NEWLINE | TAB | WHITE_SPACE => {
                    self.current_index + 1;
                    continue;
                }
                UTF8_2 => {
                    self.current_index + 2;
                    continue;
                }
                UTF8_3 => {
                    self.current_index + 3;
                    continue;
                }
                UTF8_4 => {
                    self.current_index + 4;
                    continue;
                }
                _ => return Some(c),
            }
        }
    }
}
