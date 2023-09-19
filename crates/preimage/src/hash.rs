
use sha3::{Digest, Sha3_256};

/// SHA3 256-bit hash function. Accepts a variable length byte slice and returns a 32-byte
/// (256-bit) digest.
pub fn sha3_256(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha3_256::new();
    hasher.update(data);
    hasher.finalize().into()
}
