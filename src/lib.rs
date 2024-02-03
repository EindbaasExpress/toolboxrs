pub mod internal;
mod cli;

pub use crate::internal::cidr::ipv4_to_cidr_out_loud;
pub use crate::internal::hash::hash_once_out_loud;
pub use crate::internal::base64::{process_b64_out_loud, Base64OperationType};
pub use crate::cli::{Base64Commands, Cli, Commands, CidrCommands, Algorithm};
