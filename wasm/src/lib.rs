extern crate console_error_panic_hook;
use log::info;
use shared::ipv4_to_cidr;
use shared::{hash_once, Algorithm};
use shared::{process_b64, Base64OperationType};
use std::panic;
use wasm_bindgen::prelude::*;

fn init_log() {
    // wasm_logger::init(wasm_logger::Config::default());
    // console_log::init_with_level(Level::Debug).expect("error initializing log");
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

fn str_to_b64_operation_type(input: &str) -> Base64OperationType {
    match input {
        "decode_standard" => Base64OperationType::DecodeStandard,
        "decode_urlsafe" => Base64OperationType::DecodeURLSafe,
        "encode_standard" => Base64OperationType::EncodeStandard,
        "encode_urlsafe" => Base64OperationType::EncodeURLSafe,
        _ => Base64OperationType::DecodeStandard,
    }
}

#[wasm_bindgen]
pub fn base64(input: &str, input_type: &str) -> String {
    init_log();
    info!("we're in");
    let output = process_b64(input, str_to_b64_operation_type(input_type));
    info!("{}", output);
    info!("we're out");
    output
}

#[wasm_bindgen]
pub fn cidr(input: &str) -> Result<JsValue, JsValue> {
    init_log();
    let output = ipv4_to_cidr(input);
    Ok(serde_wasm_bindgen::to_value(&output)?)
}

#[wasm_bindgen]
pub fn cidr2(input: &str) -> String {
    init_log();
    let output = ipv4_to_cidr(input);
    format!("{}", output)
}


fn str_to_hash_algo(input: &str) -> &Algorithm {
    match input {
        "Sha256" => &Algorithm::Sha256,
        "Sha384" => &Algorithm::Sha384,
        "Sha512" => &Algorithm::Sha512,
        "Sha3_256" => &Algorithm::Sha3_256,
        "Sha3_384" => &Algorithm::Sha3_384,
        "Sha3_512" => &Algorithm::Sha3_512,
        "Blake3" => &Algorithm::Blake3,
        _ => &Algorithm::Sha256,
    }
}
#[wasm_bindgen]
pub fn hash(input: &str, input_type: &str) -> String {
    hash_once(input, str_to_hash_algo(input_type))
}

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}
