use hex_literal::hex;
use sha2::{Sha256, Sha384, Sha512, Digest};
use sha3::{Sha3_256, Sha3_384, Sha3_512, Digest as Sha3Digest};
use blake3::Hasher as Blake3Hasher;
// use std::io::Read;

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
fn hash_data<H: Hasher>(hasher: &mut H, data: &[u8]) -> Vec<u8> {
    hasher.update(data);
    hasher.finalize()
}

pub enum AvailableAlgorithm {
    Sha256,
    Sha384,
    Sha512,
    Sha3_256,
    Sha3_384,
    Sha3_512,
    Blake3,
}

pub struct HashOperation {

}
impl HashOperation {
    fn create_hasher(algorithm: AvailableAlgorithm) -> Box<dyn Hasher> {
        match algorithm {
            AvailableAlgorithm::Sha256 => Box::new(Sha256::new()),
            AvailableAlgorithm::Sha384 => Box::new(Sha384::new()),
            AvailableAlgorithm::Sha512 => Box::new(Sha512::new()),
            AvailableAlgorithm::Sha3_256 => Box::new(Sha3_256::new()),
            AvailableAlgorithm::Sha3_384 => Box::new(Sha3_384::new()),
            AvailableAlgorithm::Sha3_512 => Box::new(Sha3_512::new()),
            AvailableAlgorithm::Blake3 => Box::new(Blake3Hasher::new()),
        }
    }
    
    pub fn hash(value: &str, algorithm:AvailableAlgorithm) -> String {
    
        let mut hasher = Self::create_hasher(algorithm);
    
        let data = value.as_bytes();
        hasher.update(data);
        let hash_result = hasher.finalize();
        assert_eq!(hash_result[..], hex!("
        b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9
        ")[..]);


        String::from_utf8(hash_result).expect("Our bytes should be valid utf8")

        
        // // Hash an input incrementally.
        // let mut hasher = blake3::Hasher::new();
        // hasher.update(b"foo");
        // hasher.update(b"bar");
        // hasher.update(b"baz");
        // let hash2 = hasher.finalize();
        // assert_eq!(hash1, hash2);

    }
    
    pub fn hash_out_loud(value: &str, algorithm:AvailableAlgorithm) -> () {
        let output = Self::hash(value, algorithm);
        println!("your output is: \n{}", output);

    }

}

