use cidr::{self, Ipv4Cidr};
// use ipnet::Ipv4Subnets;
use std::str::FromStr;
use regex::Regex;

pub fn info() -> String {
    return String::from("Classless Inter-Domain Routing is 
    a method for allocating IP addresses and for IP routing. 
    The Internet Engineering Task Force introduced CIDR in 1993 
    to replace the previous classful network addressing architecture 
    on the Internet.")
}
pub fn ipv4_to_cidr(ip_address: &str) -> Ipv4Cidr { 

    let cidr = match cidr::Ipv4Cidr::from_str(ip_address) {
        Ok(cidr) => cidr,
        Err(error) => {
            let message = "Your input could not be parsed as an ip address";
        
            let re = Regex::new(r"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}/\d{1,2}").unwrap();
            let re_match = match re.is_match(ip_address) {
                true => "you did match the expected regex, but",
                false => "you did not match the expected regex",
            };

            panic!("{}\n {}\n {}\n", message, re_match, error);
        }
    };
    return cidr
}
pub fn ipv4_to_cidr_out_loud(ip_address: &str) -> () {
    let cidr = ipv4_to_cidr(ip_address);

    println!("network_length                  : {}", cidr.network_length()); // called cidrnumber
    println!("cidr_range                      : {}", cidr.to_string()); // CidrRange
    println!("subnet_mask                     : {}", cidr.mask());  // subnet mask
    println!("first_ip                        : {}", cidr.first_address()); // called parsedIP and firstip
    println!("last_ip                         : {}", cidr.last_address()); // called lastip
    println!("number of ips in this cidr range: {}", cidr.iter().count()); // called lastip
    // println!("{}", cidr.()); // called

}

// fn count_ips_in_subnetmask(ip_address: &str) -> () {

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_ipv4_to_cidr_single_ip() {
        
        let value = "192.168.0.0/32"; // this is only 1 ip address
        
        let cidr = ipv4_to_cidr(value);

        let expected_parsed_ip = "192.168.0.0";
        assert_eq!(cidr.first_address().to_string(), expected_parsed_ip);
        let expected_parsed_ip_net = "192.168.0.0"; // since the cidr only contains 1 address
        assert_eq!(cidr.to_string(), expected_parsed_ip_net);
        let expected_parsed_mask = "255.255.255.255";
        assert_eq!(cidr.mask().to_string(), expected_parsed_mask);

    }
    #[test]
    fn test_ipv4_to_cidr_ip_range() {
        
        // given the following cidr range as ipv4 in a string
        let value = "192.168.0.0/16";
        // when parsed as cidr object
        let cidr = ipv4_to_cidr(value);

        // then the expected values are:
        let expected_parsed_ip = "192.168.0.0";
        assert_eq!(cidr.first_address().to_string(), expected_parsed_ip);
        let expected_parsed_ip_net = "192.168.0.0/16"; // since the cidr only contains 1 address
        assert_eq!(cidr.to_string(), expected_parsed_ip_net);
        let expected_parsed_mask = "255.255.0.0";
        assert_eq!(cidr.mask().to_string(), expected_parsed_mask);

    }

    // #[test]
    // fn test_count_ips_in_subnetmask() {
        
    //     // given the following cidr range as ipv4 in a string
    //     let value = "255.255.0.0/14";
    //     // when parsed as cidr object
    //     let cidr = count_ips_in_subnetmask(value);

    //     // then the expected values are:
    //     // let expected_parsed_ip = "192.168.0.0";
    //     // assert_eq!(cidr.first_address().to_string(), expected_parsed_ip);
    //     // let expected_parsed_ip_net = "192.168.0.0/16"; // since the cidr only contains 1 address
    //     // assert_eq!(cidr.to_string(), expected_parsed_ip_net);
    //     // let expected_parsed_mask = "255.255.0.0";
    //     // assert_eq!(cidr.mask().to_string(), expected_parsed_mask);

    // }

}