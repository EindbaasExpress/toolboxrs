mod cli;

use clap::Parser;


use toolboxrs::{ipv4_to_cidr_out_loud, hash_once_out_loud};
use toolboxrs::{Base64Commands, Cli, Commands, CidrCommands};
use toolboxrs::{process_b64_out_loud, Base64OperationType};

// use crate::cli::Base64Commands;

fn main() {
    let cli = Cli::parse();
//     println!(r"
//   _____  _____  _____  _     ______  _____ __   ________  _____ 
//  |_   _||  _  ||  _  || |    | ___ \|  _  |\ \ / /| ___ \/  ___|
//    | |  | | | || | | || |    | |_/ /| | | | \ V / | |_/ /\ `--. 
//    | |  | | | || | | || |    | ___ \| | | | /   \ |    /  `--. \
//    | |  \ \_/ /\ \_/ /| |____| |_/ /\ \_/ // /^\ \| |\ \ /\__/ /
//    \_/   \___/  \___/ \_____/\____/  \___/ \/   \/\_| \_|\____/ 
                                                                

//         ");

    match &cli.command {
        Commands::Cidr(args) => match &args.cidr_commands {
                CidrCommands::Ipaddress(args) => {
                    println!("You passed {} as an ip_address \n", args.ip_address);
                    ipv4_to_cidr_out_loud(&args.ip_address);
                }
                CidrCommands::Subnetmask(args) => {
                    println!("You passed {} as mask", args.subnetmask);
                }
        },
        Commands::Base64(args) => match &args.base64_commands {
                Base64Commands::Encode(args) => {
                    println!("You passed a value to be base64-encoded, the url_safe option is set to {}", args.url_safe);
                    if args.url_safe {
                        process_b64_out_loud(&args.value, Base64OperationType::EncodeURLSafe);
                    } else {
                        process_b64_out_loud(&args.value, Base64OperationType::EncodeStandard);
                    }   
                }
                Base64Commands::Decode(args) => {
                    println!("You passed a value to be base64-decoded, the url_safe option is set to {}", args.url_safe);
                    if args.url_safe {
                        process_b64_out_loud(&args.value, Base64OperationType::DecodeURLSafe);
                    } else {
                        process_b64_out_loud(&args.value, Base64OperationType::DecodeStandard);
                    }   
                }
        }
        Commands::Hash(args) => {
                println!("You passed {} as a value to hash \n", args.value);
                hash_once_out_loud(&args.value, &args.algorithm);
    },
    }
}
