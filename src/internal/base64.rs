use base64::{Engine as _, engine::general_purpose};

pub enum Base64OperationType {
    DecodeStandard,
    DecodeURLSafe,
    EncodeStandard,
    EncodeURLSafe,
}

pub fn info() -> String {
    return String::from("
    Base64 is a group of similar binary-to-text encoding schemes 
    that represent binary data in an ASCII string format by 
    translating it into a radix-64 representation. 
    The term Base64 originates from a specific MIME content transfer encoding. 
    Each base64 digit represents exactly 6 bits of data.")
}

/// Function that performs standard Base64 encoding and decoding and URL-safe encoding decoding. 
/// Requires Base64Operation enum values as input.
pub fn process_b64(value :&str, operation_type:Base64OperationType) -> String {
    
    /// removing the padding by replacing the input string with everything
    /// except the =-characters
    fn removing_padding(input:&str) -> String {
        use regex::Regex;
    let re = Regex::new(r"([^=]*)(=*)$").unwrap();
    let result = re.replace_all(input, "$1");
    result.to_string()
    }

    let output: String = match operation_type {
        Base64OperationType::DecodeURLSafe => match general_purpose::URL_SAFE_NO_PAD.decode(
            removing_padding(value)
        ) {
                Ok(vec) =>  String::from_utf8(vec).unwrap(),
                Err(error) => panic!("this error was thrown: {}", error)
            },
            Base64OperationType::EncodeURLSafe => general_purpose::URL_SAFE_NO_PAD.encode(value.as_bytes()),
            Base64OperationType::DecodeStandard => match general_purpose::STANDARD_NO_PAD.decode(value) {
                Ok(vec) =>  String::from_utf8(vec).unwrap(),
                Err(error) => panic!("this error was thrown: {}", error)
            },
            Base64OperationType::EncodeStandard => general_purpose::STANDARD_NO_PAD.encode(value.as_bytes()),
    };
    return output
}
pub fn process_b64_out_loud(value: &str, operation_type:Base64OperationType) -> () {
    let output = process_b64(value, operation_type);
    println!("your output is: \n{}", output);
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_base64encode_decode() {
        let orig = "data";
        let encoded: String = process_b64(orig, Base64OperationType::EncodeStandard);
        assert_eq!("ZGF0YQ", encoded);
        assert_eq!(orig, process_b64(&encoded, Base64OperationType::DecodeStandard));
        
        let encoded2: String = process_b64(orig, Base64OperationType::EncodeURLSafe);
        assert_eq!("ZGF0YQ", encoded2);
        assert_eq!(orig, process_b64(&encoded2, Base64OperationType::DecodeURLSafe));

        let encoded_incorrect_padding = "aHR0cHM6Ly9iaXQubHkvM21NMDZmRQ==";
        let expected_str = "https://bit.ly/3mM06fE";
        assert_eq!(expected_str, process_b64(&encoded_incorrect_padding, Base64OperationType::DecodeURLSafe));

    }
}