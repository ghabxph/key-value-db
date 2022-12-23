use crate::{split, join};

// Get message from given signed message
pub fn get_message(signed_message: &[u8]) -> Vec<u8> {
    
    // Get tokens
    let mut tokens = split(signed_message, b'\x0A');

    // Pop off the signature
    tokens.pop();

    // Gets the message by joining the message minus the signature
    let message = join(tokens, b'\x0A');

    // Return message
    return message;
}

#[test]
fn test_get_meessage() {
    let signed_message = b"AUTHORIZE\x0APUBLIC KEY \"\"\x0AAS MASTER\x0AAUTHORIZED BY \"\"\x0ASIGNATURE \"Gabriel\"";
    let result = get_message(signed_message);
    assert!(result.eq(b"AUTHORIZE\x0APUBLIC KEY \"\"\x0AAS MASTER\x0AAUTHORIZED BY \"\""));
}