use sha2::{Sha256, Digest};

/// Digest given message into fixed 256-byte sha256 hash encoded in base58 string
pub fn b58_sha256(message: &[u8]) -> String {

    // Sha256 hasher instance
    let mut hasher = Sha256::new();

    // Get the hash of the input message
    hasher.update(message);
    let result = hasher.finalize();
    let hash: [u8;32] = result.to_vec().try_into().unwrap();

    // Return the base58 encoded hash
    bs58::encode(hash).into_string()
}

#[test]
pub fn test_bs58_sha256() {
    let result = b58_sha256(b"My nice message");
    assert!(result.eq("6beRDP1wrSrpmv9ByVGJqEi2h9juzmZBfKA8gF4Uhy2d"));
}