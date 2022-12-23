use ed25519_dalek::*;

use crate::{CryptoError};

pub fn ed25519_verify(public_key: &[u8], msg: &[u8], signature: [u8; 64]) -> Result<(), CryptoError> {
    let signature = signatory::ed25519::Signature::from(signature);
    let verifier = signatory::ed25519::VerifyingKey::from_bytes(public_key).unwrap();
    match verifier.verify(msg, &signature) {
        Ok(_) => Ok(()),
        Err(_) => Err(CryptoError::SignatureVerificationFailed),
    }
}

#[test]
pub fn test_ed25519_verify() {
    // let mut csprng = rand_core::OsRng;
    // let keypair = ed25519_dalek::Keypair::generate(&mut csprng);
    let keypair = crate::test_keypair();
    // ed25519_dalek::SecretKey::from())
    // let keypair = ed25519_dalek::Keypair::from();
    let public_key = keypair.public.to_bytes();
    let message = b"A touching message";
    let signature = keypair.sign(message).to_bytes();
    ed25519_verify(&public_key, message, signature).unwrap();
}
