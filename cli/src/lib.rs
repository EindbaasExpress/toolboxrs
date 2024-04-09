use clap::{Args, Parser, Subcommand, ArgAction};
use shared::Algorithm;

#[derive(Parser)]
#[clap(name = "toolboxrs")]
pub struct Cli {
    // Subcommands
    // You can only have one subcommand section
    // so we point this to the Commands struct
    #[structopt(subcommand)] 
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// get information on a provided cidr range
    #[clap(name = "cidr")]
    Cidr(CidrCommand),
    /// base64-encode, -decode strings, optionally url-safe
    #[clap(name = "base64")]
    Base64(Base64Command),
    /// hash a string using one of the listed algorithms
    #[clap(name = "hash")]
    Hash(HashArgs),
    /// start a toolboxrs-server with a UI on port 3030
    #[clap(name = "server")]
    Server(ServerArgs),
}

// The 'cidr' command itself
// The only way this differs from the Commands struct
// is that it directes clap::Parser, instead of clap::Subcommand
// This is so it gains the ability to auto-parse into
// the structure tree as a clap::Command with args, instead of as
// a clap::Args object
#[derive(Parser)]
pub struct CidrCommand {
         #[structopt(subcommand)]
        pub cidr_commands: CidrCommands,
}
// The command enum for the 'cidr' command
#[derive(Subcommand)]
pub enum CidrCommands {
    // Here we pass in the args for the 'cidr ipaddress' command
    Ipaddress(IpArgs),
    Subnetmask(MaskArgs),
}

#[derive(Args)]
pub struct IpArgs {
    pub ip_address: String,
}

#[derive(Args)]
pub struct MaskArgs {
    pub subnetmask: String,
}

#[derive(Parser)]
pub struct Base64Command {
         #[structopt(subcommand)]
        pub base64_commands: Base64Commands,
}

#[derive(Args)]
pub struct Base64Args {
    pub value: String,
    #[arg(short, long, action=ArgAction::SetTrue)]
    pub url_safe: bool,
    // #[arg(short, long, action=ArgAction::SetTrue)]
    // pub padding: bool,
}

// The command enum for the 'base64' command
#[derive(Subcommand)]
pub enum Base64Commands {
    // Here we pass in the args for the Base64 commands
    Encode(Base64Args),
    Decode(Base64Args),
}

#[derive(Args)]
pub struct HashArgs {
    pub value: String,
    #[clap(value_enum, default_value_t=Algorithm::Sha256)]
    pub algorithm: Algorithm,
}

#[derive(Args)]
pub struct ServerArgs {
    /// sets the port of the local server
    #[arg(short, long, default_value="3030")]
    pub port: String,
}
