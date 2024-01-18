use base64::{Engine as _, engine::general_purpose};
// use std::str::from_utf8;

pub enum Base64OperationType {
    DecodeStandard,
    DecodeURLSafe,
    EncodeStandard,
    EncodeURLSafe,
}

pub struct Base64Operation {
}
impl Base64Operation {
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
    pub fn process(value :&str, operation_type:Base64OperationType) -> String {
        let output: String = match operation_type {
            Base64OperationType::DecodeURLSafe => match general_purpose::URL_SAFE_NO_PAD.decode(value) {
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
    pub fn process_out_loud(value: &str, operation_type:Base64OperationType) -> () {
        let output = Self::process(value, operation_type);
        println!("your output is: \n{}", output);
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_base64encode_decode() {
        let orig = "data";
        let encoded: String = Base64Operation::process(orig, Base64OperationType::EncodeStandard);
        assert_eq!("ZGF0YQ", encoded);
        assert_eq!(orig, Base64Operation::process(&encoded, Base64OperationType::DecodeStandard));
        
        let encoded2: String = Base64Operation::process(orig, Base64OperationType::EncodeURLSafe);
        assert_eq!("ZGF0YQ", encoded2);
        assert_eq!(orig, Base64Operation::process(&encoded2, Base64OperationType::DecodeURLSafe));

    }
}