use cidr::Ipv4Cidr;
use regex::Regex;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::{fmt, str::FromStr};

// Classless Inter-Domain Routing is
// a method for allocating IP addresses and for IP routing.
// The Internet Engineering Task Force introduced CIDR in 1993
// to replace the previous classful network addressing architecture
// on the Internet.

#[derive(Debug)]
pub struct CidrRangeOutput {
    cidr: Option<Ipv4Cidr>,
    message: Option<String>,
}
impl CidrRangeOutput {
    fn new( cidr: Option<Ipv4Cidr>, message: Option<String>) -> Self {
        Self { cidr, message }
    }
}
impl fmt::Display for CidrRangeOutput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.cidr {
            None => match &self.message {
                None => panic!("No CIDR, but no message either, bug in application code"),
                Some(message) => write!(f, "{}", message),
            }
            Some(cidr) => write!(
                f,
                "
            network_length                  : {}
            cidr_range                      : {}
            subnet_mask                     : {}
            first_ip                        : {}
            last_ip                         : {}
            number of ips in this cidr range: {}
            ",
                cidr.network_length(),
                cidr.to_string(),
                cidr.mask(),
                cidr.first_address(),
                cidr.last_address(),
                cidr.iter().count()
            ),
        }
    }
}
impl Serialize for CidrRangeOutput {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self.cidr {
            None => match &self.message {
                None => panic!("No CIDR, but no message either, bug in application code"),
                Some(message) => {
                    let mut state = serializer.serialize_struct("CidrRangeOutput", 6)?;
                    state.serialize_field("message", &message)?;
                    state.end()
                }
            }
            Some(cidr) => {
                let mut state = serializer.serialize_struct("CidrRangeOutput", 6)?;
                state.serialize_field("network_length", &cidr.network_length())?;
                state.serialize_field("cidr_range", &cidr.to_string())?;
                state.serialize_field("subnet_mask",&cidr.mask())?;
                state.serialize_field("first_ip", &cidr.first_address())?;
                state.serialize_field("last_ip", &cidr.last_address())?;
                state.serialize_field("num_of_ips", &cidr.iter().count())?;
                state.serialize_field("message", "")?;
                
                state.end()
            }
            
        }

        
    }
}

pub fn ipv4_to_cidr(ip_address: &str) -> CidrRangeOutput {
    match cidr::Ipv4Cidr::from_str(ip_address) {
        Ok(cidr) => CidrRangeOutput::new(Some(cidr), None),
        Err(error) => {
            let message = "Your input could not be parsed as an ip address.";


            let re = Regex::new(r"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}/\d{1,2}").unwrap();
            let re_match = match re.is_match(ip_address) {
                true => "you did match the expected regex(expecting X.X.X.X/Y format), but somehow it's not valid.",
                false => "you did not match the expected regex, expecting X.X.X.X/Y format.",
            };
            let complete_error_message = format!("{} {} Original Error was: {}", message, re_match, error);
            CidrRangeOutput::new(None, Some(complete_error_message))
        }
    }
}

pub fn ipv4_to_cidr_out_loud(ip_address: &str) {
    let cidr_range_output = ipv4_to_cidr(ip_address);
    println!("{}", cidr_range_output)
    
}

// fn count_ips_in_subnetmask(ip_address: &str) -> () {

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use serde_json::json;

    #[test]
    fn test_print_output() {
        let value = "192.168.0.0/32"; // this is only 1 ip address

        let cidr_range_output = ipv4_to_cidr(value);

        let expected_string = "
        network_length                  : 32
        cidr_range                      : 192.168.0.0
        subnet_mask                     : 255.255.255.255
        first_ip                        : 192.168.0.0
        last_ip                         : 192.168.0.0
        number of ips in this cidr range: 1
        ";


        assert_eq!(cidr_range_output.to_string(), expected_string);
        

    }
    #[test]
    fn test_json_serializer(){
        let value = "192.168.0.0/32"; // this is only 1 ip address

        let cidr_range_output = ipv4_to_cidr(value);
        
        let json = serde_json::to_string(&cidr_range_output).unwrap();
        let parsed_json: serde_json::Value = serde_json::from_str(&json).unwrap();
        // Expected JSON string
        let expected_json = json!({
            "network_length": 32,
            "cidr_range": "192.168.0.0",
            "subnet_mask": "255.255.255.255",
            "first_ip": "192.168.0.0",
            "last_ip": "192.168.0.0",
            "num_of_ips": 1,
        });

        // Assert that the serialized JSON matches the expected JSON
        assert_eq!(parsed_json, expected_json);
    }

    #[test]
    fn test_ipv4_to_cidr_single_ip() {
        let value = "192.168.0.0/32"; // this is only 1 ip address

        let CidrRangeOutput {cidr, message: _} = ipv4_to_cidr(value);

        let expected_parsed_ip = "192.168.0.0";
        assert_eq!(cidr.expect("cidr was None").first_address().to_string(), expected_parsed_ip);
        let expected_parsed_ip_net = "192.168.0.0"; // since the cidr only contains 1 address
        assert_eq!(cidr.expect("cidr was None").to_string(), expected_parsed_ip_net);
        let expected_parsed_mask = "255.255.255.255";
        assert_eq!(cidr.expect("cidr was None").mask().to_string(), expected_parsed_mask);
    }
    #[test]
    fn test_ipv4_to_cidr_ip_range() {
        // given the following cidr range as ipv4 in a string
        let value = "192.168.0.0/16";
        // when parsed as cidr object
        let CidrRangeOutput {cidr, message: _} = ipv4_to_cidr(value);

        // then the expected values are:
        let expected_parsed_ip = "192.168.0.0";
        assert_eq!(cidr.expect("cidr was None").first_address().to_string(), expected_parsed_ip);
        let expected_parsed_ip_net = "192.168.0.0/16"; // since the cidr only contains 1 address
        assert_eq!(cidr.expect("cidr was None").to_string(), expected_parsed_ip_net);
        let expected_parsed_mask = "255.255.0.0";
        assert_eq!(cidr.expect("cidr was None").mask().to_string(), expected_parsed_mask);
    }
    #[test]
    fn test_returning_message_when_cidr_range_invalid() {
        // given the following cidr range as ipv4 in a string
        let value = "10.0.0.0/3";
        // when parsed as cidr object
        let CidrRangeOutput {cidr: _, message} = ipv4_to_cidr(value);

        // then the expected values are:
        let expected_message = "Your input could not be parsed as an ip address. \
        you did match the expected regex(expecting X.X.X.X/Y format), but somehow it's not valid. \
        Original Error was: host part of address was not zero";
        assert_eq!(message.expect("some message expected"), expected_message);

    }
}
