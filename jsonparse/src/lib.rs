mod utils;

#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::sync::Mutex;

use wasm_bindgen::prelude::*;

lazy_static! {
    // <ref_id, JsonParser>
    static ref PARSER_MAP: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

#[wasm_bindgen]
/// Intializes the JsonParser and returns a ref_id for the new parser.
/// An optional argumant provides the path to the array to stream. Omit if array.
pub fn create_json_parser(path_to_array: Option<Box<[JsValue]>>) -> String {
    todo!("Initialize");
}

#[wasm_bindgen(catch)]
/// Appends additional streaming data to the JsonParser.
/// Throws NOT_FOUND or FATAL.
pub fn push(ref_id: String, uint8_array: Box<[u8]>) -> Result<(), JsValue> {
    if let Ok(mut map) = PARSER_MAP.lock() {
        if let Some(mut parser) = map.get_mut(&ref_id) {
            // todo!("Append uint8_array into the parser.");
            return Ok(());
        }
        return Err(JsValue::from_str("NOT_FOUND"));
    }
    Err(JsValue::from_str("FATAL"))
}

#[wasm_bindgen(catch)]
/// Attempts to collect and return the next item in the streaming data.
/// Throws PENDING, NOT_FOUND or FATAL.
pub fn poll(ref_id: String) -> Result<Box<[u8]>, JsValue> {
    if let Ok(mut map) = PARSER_MAP.lock() {
        if let Some(mut parser) = map.get_mut(&ref_id) {
            // todo!("Not implemented");
            // return Ok(Box::new([1]));
            return Err(JsValue::from_str("PENDING"));
        }
        return Err(JsValue::from_str("NOT_FOUND"));
    }
    Err(JsValue::from_str("FATAL"))
}

#[wasm_bindgen]
/// Call when the JsonParser is no longer needed.
/// Fails silently if ref_id does not exist.
pub fn destroy(ref_id: String) {
    if let Ok(mut parser) = PARSER_MAP.lock() {
        parser.remove(&ref_id);
    }
}
