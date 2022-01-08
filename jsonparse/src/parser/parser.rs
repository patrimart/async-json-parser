use super::constants::*;

pub enum PollResponse {
    Next(Box<[u8]>),
    Pending,
    Done,
}

#[derive(Debug, Clone)]
pub struct Parser {
    path_to_array: Vec<String>,
    state: ParseState,
    current_path: Vec<String>,
    depth_stack: Vec<u8>,
    current_index: u32,
    stream: Vec<u8>,
}

impl Parser {
    pub fn new(path_to_array: Vec<String>) -> Self {
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

    pub fn poll(&self) -> PollResponse {
        PollResponse::Pending
    }
}
