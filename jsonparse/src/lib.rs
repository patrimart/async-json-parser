mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn create_parser(path_to_array: Option<Box<[JsValue]>>) -> String {
    todo!("");
}

#[wasm_bindgen]
pub fn write(handle_id: String, uint8_array: Box<[u8]>) {
    todo!("");
}

#[wasm_bindgen]
pub fn poll(handle_id: String) -> Option<Box<[u8]>> {
    todo!("");
}
