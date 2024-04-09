use wasm_bindgen::prelude::*;
use shared::{process_b64_out_loud, Base64OperationType};


fn str_to_b64_operation_type(input:&str) -> Base64OperationType {
    match input {
        "decode_standard" => Base64OperationType::DecodeStandard,
        "decode_urlsafe" => Base64OperationType::DecodeURLSafe,
        "encode_standard" => Base64OperationType::EncodeStandard,
        "encode_urlsafe" => Base64OperationType::EncodeURLSafe,
        _ => Base64OperationType::DecodeStandard
    }
}

#[wasm_bindgen]
pub fn base64(input: &str,input_type:&str) {
    process_b64_out_loud(
        input,
        str_to_b64_operation_type(input_type)
    )
}

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}