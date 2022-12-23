use crate::{split, LexError, Sha256Hash};

// Get authority from given signed message
pub fn get_authority(signed_message: &[u8]) -> Result<Sha256Hash, LexError> {
    
    // Get tokens
    let mut tokens = split(signed_message, b'\x0A');

    // Pop off the signature layer
    tokens.pop().unwrap();

    // Pop off the authority layer
    let authority_layer = tokens.pop().unwrap();

    // Split tokens by space
    let tokens = split(authority_layer.as_slice(), b' ');

    let authorize_token = tokens.get(0).unwrap();
    let by_token = tokens.get(1).unwrap();
    let authority_string = tokens.get(2).unwrap();

    // Throw error if token is incorrect
    if authorize_token.eq(b"AUTHORIZED") == false || by_token.eq(b"BY") == false {
        return Err(LexError::ExpectsTokenToBeAuthorizedBy)
    }

    // Finally, get the decoded authority
    let tokens = split(authority_string.as_slice(), b'"');
    let authority_bs58 = tokens.get(1).unwrap();
    let authority = bs58::decode(authority_bs58).into_vec().unwrap();

    // Return authority
    return Ok(authority.as_slice().try_into().unwrap());
}

#[test]
fn test_get_authority() {
    let signed_message = b"AUTHORIZE\x0APUBLIC KEY \"\"\x0AAS MASTER\x0AAUTHORIZED BY \"11111111111111111111111111111111\"\x0ASIGNATURE \"Gabriel\"";
    let result = get_authority(signed_message).unwrap();
    assert!(result.eq(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"));
}