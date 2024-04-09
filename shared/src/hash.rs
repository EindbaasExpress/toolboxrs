use sha2::{Sha256, Sha384, Sha512, Digest};
use sha3::{Sha3_256, Sha3_384, Sha3_512, Digest as Sha3Digest};
use blake3::Hasher as Blake3Hasher;
use crate::cli_model::Algorithm;

// A trait object that abstracts over types implementing the Digest trait
trait Hasher {
    fn update(&mut self, data: &[u8]);
    fn finalize(&mut self) -> Vec<u8>;
}

// Implement Hasher for Sha256
impl Hasher for Sha256 {
    fn update(&mut self, data: &[u8]) {
        Digest::update(self, data);
    }

    fn finalize(&mut self) -> Vec<u8> {
        Digest::finalize_reset(self).to_vec()
    }
}

// Implement Hasher for Sha384
impl Hasher for Sha384 {
    fn update(&mut self, data: &[u8]) {
        Digest::update(self, data);
    }

    fn finalize(&mut self) -> Vec<u8> {
        Digest::finalize_reset(self).to_vec()
    }
}

// Implement Hasher for Sha512
impl Hasher for Sha512 {
    fn update(&mut self, data: &[u8]) {
        Digest::update(self, data);
    }

    fn finalize(&mut self) -> Vec<u8> {
        Digest::finalize_reset(self).to_vec()
    }
}

// Implement Hasher for Sha3_256
impl Hasher for Sha3_256 {
    fn update(&mut self, data: &[u8]) {
        Sha3Digest::update(self, data);
    }

    fn finalize(&mut self) -> Vec<u8> {
        Sha3Digest::finalize_reset(self).to_vec()
    }
}

// Implement Hasher for Sha3_384
impl Hasher for Sha3_384 {
    fn update(&mut self, data: &[u8]) {
        Sha3Digest::update(self, data);
    }

    fn finalize(&mut self) -> Vec<u8> {
        Sha3Digest::finalize_reset(self).to_vec()
    }
}

// Implement Hasher for Sha3_512
impl Hasher for Sha3_512 {
    fn update(&mut self, data: &[u8]) {
        Sha3Digest::update(self, data);
    }

    fn finalize(&mut self) -> Vec<u8> {
        Sha3Digest::finalize_reset(self).to_vec()
    }
}

// Implement Hasher for Blake3
impl Hasher for Blake3Hasher {
    fn update(&mut self, data: &[u8]) {
        Self::update(self, data);
    }

    fn finalize(&mut self) -> Vec<u8> {
        Self::finalize(self).as_bytes().to_vec()
    }
}

// A function that takes any type implementing the Hasher trait
// fn hash_data<H: Hasher>(hasher: &mut H, data: &[u8]) -> Vec<u8> {
//     hasher.update(data);
//     hasher.finalize()
// }


struct HashOperator {
    hasher: Box<dyn Hasher>
}
impl HashOperator {
    
    /// main method, contains an implementation for every defined
    /// variant of the cli.Algorithm-enum
    fn new(algorithm: &Algorithm) -> HashOperator {
        match algorithm {
            Algorithm::Sha256 => HashOperator{hasher: Box::new(Sha256::new())},
            Algorithm::Sha384 => HashOperator{hasher: Box::new(Sha384::new())},
            Algorithm::Sha512 => HashOperator{hasher: Box::new(Sha512::new())},
            Algorithm::Sha3_256 => HashOperator{hasher: Box::new(Sha3_256::new())},
            Algorithm::Sha3_384 => HashOperator{hasher: Box::new(Sha3_384::new())},
            Algorithm::Sha3_512 => HashOperator{hasher: Box::new(Sha3_512::new())},
            Algorithm::Blake3 => HashOperator{hasher: Box::new(Blake3Hasher::new())},
        }
    }
}

/// this is the main function in this module
/// the Algorithm is an enum defined in the cli-module
/// because of clap-restrictions. 
pub fn hash_once(value: &str, algorithm: &Algorithm) -> String {
    let mut operator = HashOperator::new(algorithm);
    
    let data = value.as_bytes();
    operator.hasher.update(data);
    let hash_result = operator.hasher.finalize();

    // return output
    let hex_string: String = hash_result.iter().map(|byte| format!("{:02x}", byte)).collect();

    return hex_string

}

pub fn hash_once_out_loud(value: &str, algorithm: &Algorithm) -> () {
    let output = hash_once(value, algorithm);
    println!("your output is: \n{}", output);

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        // and validate the handling of the algorithms, just in case
        let algorithms = [
            (Algorithm::Sha256, "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"),
            (Algorithm::Sha384, "cb00753f45a35e8bb5a03d699ac65007272c32ab0eded1631a8b605a43ff5bed8086072ba1e7cc2358baeca134c825a7"),
            (Algorithm::Sha512, "ddaf35a193617abacc417349ae20413112e6fa4e89a97ea20a9eeee64b55d39a2192992a274fc1a836ba3c23a3feebbd454d4423643ce80e2a9ac94fa54ca49f"),
            (Algorithm::Sha3_256, "3a985da74fe225b2045c172d6bd390bd855f086e3e9d525b46bfe24511431532"),
            (Algorithm::Sha3_384, "ec01498288516fc926459f58e2c6ad8df9b473cb0fc08c2596da7cf0e49be4b298d88cea927ac7f539f1edf228376d25"),
            (Algorithm::Sha3_512, "b751850b1a57168a5693cd924b6b096e08f621827444f70d884f5d0240d2712e10e116e9192af3c91a7ec57647e3934057340b4cf408d5a56592f8274eec53f0"),
            (Algorithm::Blake3, "6437b3ac38465133ffb63b75273a8db548c558465d79db03fd359c6cd5bd9d85"),
        ];
        for (algorithm, expected_output) in algorithms.into_iter(){
            let output = hash_once("abc", &algorithm);
            assert_eq!(output, String::from(expected_output))
        }
    }

}