use super::constants::*;

pub enum PollResponse {
    Next(&[u8]),
    Pending,
    Done,
}

pub struct Parser {
    state: ParseState,
    current_path: Vec<String>,
    depth_stack: Vec<u8>,
    current_index: u32,
    stream: Vec<u8>,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            state: ParseState::AtStart,
            current_path: Vec::new(),
            depth_stack: Vec::new(),
            current_index: 0,
            stream: Vec::new(),,
        }
    }

    pub fn push(&mut self, arr: &[u8]) {}

    pub fn poll(&self) -> PollResponse {}
}
