mod cidr;
mod hash;
mod base64;
mod cli_model;

pub use cidr::ipv4_to_cidr_out_loud;
pub use hash::hash_once_out_loud;
pub use base64::{process_b64_out_loud, Base64OperationType};
pub use cli_model::Algorithm;
