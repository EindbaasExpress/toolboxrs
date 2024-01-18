mod cidr;
mod base64;
mod cli;
mod hash;

pub use crate::cidr::ipv4_to_cidr_out_loud;
pub use crate::base64::{Base64Operation, Base64OperationType};
pub use crate::cli::{Base64Commands, Cli, Commands, CidrCommands};
