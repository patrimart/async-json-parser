mod utils;

#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

use wasm_bindgen::prelude::*;

lazy_static! {
    // <ref_id, JsonParser>
    static ref PARSER_MAP: HashMap<String, String> = HashMap::new();
}

#[wasm_bindgen]
pub enum PollResponse {
    Pending = 0,
    Done = 1,
}

#[wasm_bindgen]
/// Intializes the JsonParser and returns a ref_id for the new parser.
/// An optional argumant provides the path to the array to stream. Omit if array.
pub fn create_json_parser(path_to_array: Option<Box<[JsValue]>>) -> String {
    todo!("Not implemented");
}

#[wasm_bindgen(catch)]
/// Appends additional streaming data to the JsonParser.
/// Throws PollResponse.
pub fn push(ref_id: String, uint8_array: Box<[u8]>) -> Result<(), JsValue> {
    todo!("Not implemented");
}

#[wasm_bindgen(catch)]
/// Attempts to collect and return the next item in the streaming data.
/// Throws if ref_id does not exist.
pub fn poll(ref_id: String) -> Result<Box<[u8]>, JsValue> {
    todo!("Not implemented");
}

#[wasm_bindgen]
/// Call when the JsonParser is no longer needed.
/// Fails silently if ref_id does not exist.
pub fn destroy(ref_id: String) {
    
    todo!("Not implemented");
}
