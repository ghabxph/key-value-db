use ed25519_dalek::{PublicKey, SecretKey, Keypair};

pub fn test_keypair() -> Keypair {
    let secret_bytes: [u8;32] = bs58::decode("6BGdEKrxDUwhNbm1gEKsHs82GUbmYtoMrd2JVgoLKgF9").into_vec().unwrap().as_slice().try_into().unwrap();
    let public: PublicKey = (&SecretKey::from_bytes(&secret_bytes).unwrap()).into();
    let secret = SecretKey::from_bytes(&secret_bytes).unwrap();
    Keypair { secret, public }
}