
#[derive(Debug)]
pub enum MessageVerifierError {
    CannotGetSignature,
    VerificationFailed,
    NotYetImplemented,
}

#[derive(Debug)]
pub enum LexError {
    ExpectsTokenToBeSignature,
    ExpectsTokenToBeAuthorizedBy,
}

#[derive(Debug)]
pub enum DBError {
    KeyDoesNotExist,
}

#[derive(Debug)]
pub enum CryptoError {
    SignatureVerificationFailed,
}
