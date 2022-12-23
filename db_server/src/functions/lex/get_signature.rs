use crate::{split, LexError, Signature};

// Get signature from given signed message
pub fn get_signature(signed_message: &[u8]) -> Result<Signature, LexError> {
    
    // Get tokens
    let mut tokens = split(signed_message, b'\x0A');

    // Pop off the signature layer
    let signature_layer = tokens.pop().unwrap();

    // Split tokens by space
    let tokens = split(signature_layer.as_slice(), b' ');

    let signature_token = tokens.get(0).unwrap();
    let signature_string = tokens.get(1).unwrap();

    // Throw error if token is incorrect
    if signature_token.eq(b"SIGNATURE") == false {
        return Err(LexError::ExpectsTokenToBeSignature)
    }

    // Finally, get the signature value
    let tokens = split(signature_string.as_slice(), b'"');
    let signature = tokens.get(1).unwrap();
    let signature =  bs58::decode(signature).into_vec().unwrap().try_into().unwrap();

    // Return signature
    return Ok(signature);
}

#[test]
fn test_get_signature() {
    let signed_message = b"AUTHORIZE\x0APUBLIC KEY \"\"\x0AAS MASTER\x0AAUTHORIZED BY \"EqVLzPPJyC4LgKgcpkXcQ49XU6UaRjyvq3mMZMfAcX6M\"\x0ASIGNATURE \"5ah1DX9RAu3B2zQtf142YXXhFhoePqhQQe2nrGC61UNjr4Xrf6Y3aS9ajcQJoPoodmgNd42sSM6oEcYNhsydckto\"";
    let result = get_signature(signed_message).unwrap();
    assert!(bs58::encode(result).into_string().eq("5ah1DX9RAu3B2zQtf142YXXhFhoePqhQQe2nrGC61UNjr4Xrf6Y3aS9ajcQJoPoodmgNd42sSM6oEcYNhsydckto"));
}