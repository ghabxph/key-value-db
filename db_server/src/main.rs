pub mod functions;
pub use functions::*;

pub mod error;
pub use error::*;

pub mod types;
pub use types::*;

fn main() {
    generate_first_master();
    println!("Hello, world!");
}

/// Verifies raw message is authorized to execute command
pub struct MessageVerifier {
    /// Public key hash
    pub public_key_hash: Sha256Hash,
    /// Partially parsed message removing the signature part that is to be
    /// verified across given signature
    pub message: Vec<u8>,
    /// Signature used to verify the message
    pub signature: Signature,
}

impl MessageVerifier {

    /// Creates new instance of this class from given raw text message
    /// This pre-parses the message into message and signature part.
    pub fn new(signed_message: &[u8]) -> Result<MessageVerifier, MessageVerifierError> {

        // Get message
        let message = get_message(signed_message);

        // Get signature
        let signature = get_signature(signed_message).unwrap();

        // Get authorized by (public key)
        let public_key_hash = get_authority(signed_message).unwrap();

        // Return the message and signature
        Ok(MessageVerifier { public_key_hash, message, signature })
    }

    /// Verify that the given message is authorized to perform the requested
    /// command.
    pub fn verify(&self) -> Result<(), MessageVerifierError> {
        // Fetch public key
        let public_key = bs58::encode(self.public_key_hash).into_string();
        let public_key = get("db/system/auth/master".to_string(), public_key).unwrap();

        // Get message as slice
        let msg = self.message.as_slice();

        // Get signature as slice
        let signature = self.signature.as_slice();

        // Returns empty tuple if signature is correct. Returns error if failed
        match ed25519_verify(&public_key, msg, signature.try_into().unwrap()) {
            Ok(_) => Ok(()),
            Err(_) => Err(MessageVerifierError::VerificationFailed),
        }
    }

    pub fn to_command(&self) -> Result<Command, MessageVerifierError> {
        let command = split(&self.message.as_slice(), b'\x0A');
        let command = String::from_utf8_lossy(command.get(0).unwrap()).to_string();
        if command.eq("AUTHORIZE") {
            return Ok(get_authorize_command(&self.message))
        } else if command.eq("REVOKE") {
            return Err(MessageVerifierError::NotYetImplemented)
        } else if command.eq("GET") {
            return Err(MessageVerifierError::NotYetImplemented)
        } else if command.eq("SET") {
            return Err(MessageVerifierError::NotYetImplemented)
        } else {
            return Err(MessageVerifierError::NotYetImplemented)
        }
    }
}

#[test]
fn test_message_verifier() {
    let command = b"AUTHORIZE\x0APUBLIC KEY \"\"\x0AAS MASTER\x0AAUTHORIZED BY \"EqVLzPPJyC4LgKgcpkXcQ49XU6UaRjyvq3mMZMfAcX6M\"\x0ASIGNATURE \"5ah1DX9RAu3B2zQtf142YXXhFhoePqhQQe2nrGC61UNjr4Xrf6Y3aS9ajcQJoPoodmgNd42sSM6oEcYNhsydckto\"";
    let verifier = MessageVerifier::new(command).unwrap();
    verifier.verify().unwrap();
    let _command = verifier.to_command().unwrap();
}
