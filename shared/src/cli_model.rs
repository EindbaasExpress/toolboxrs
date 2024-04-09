#[derive(Debug, clap::ValueEnum, Clone)]
pub enum Algorithm {
    Sha256,
    Sha384,
    Sha512,
    Sha3_256,
    Sha3_384,
    Sha3_512,
    Blake3,
}