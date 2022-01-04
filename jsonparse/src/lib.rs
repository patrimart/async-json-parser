mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub enum PollResponse {
    Pending = 0,
    Done = 1,
}

#[wasm_bindgen]
pub fn create_parser(path_to_array: Option<Box<[JsValue]>>) -> String {
    todo!("");
}

#[wasm_bindgen]
pub fn write(handle_id: String, uint8_array: Box<[u8]>) {
    todo!("");
}

#[wasm_bindgen(catch)]
pub fn poll(handle_id: String) -> Result<Box<[u8]>, JsValue> {
    todo!("");
}

#[wasm_bindgen]
pub fn cleanup(handle_id: String) {
    todo!("");
}
