mod base64;
mod cidr;
mod cli_model;
mod hash;

pub use base64::{process_b64, process_b64_out_loud, Base64OperationType};
pub use cidr::{ipv4_to_cidr, ipv4_to_cidr_out_loud};
pub use cli_model::Algorithm;
pub use hash::{hash_once, hash_once_out_loud};
