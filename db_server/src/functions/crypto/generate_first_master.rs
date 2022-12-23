use std::{path::Path, fs::{create_dir_all, read_dir}, os::unix::prelude::OsStrExt};
use crate::{split, b58_sha256, set, test_keypair};

pub fn generate_first_master() {
    let path_str = "db/system/auth/master".to_owned();
    let path = Path::new(&path_str);

    // If path does not exist, then create
    if path.exists() == false {
        create_dir_all(path).unwrap();
    }

    let mut has_master = false;
    for file in read_dir(path).unwrap() {
        let file_name = split(file.unwrap().file_name().as_bytes(), b'.');
        if file_name.get(1).unwrap().eq(b"dat") {
            has_master = true;
            break;
        }
    }

    if has_master == false {
        println!("Generating master private key for the first time...");
        // let mut csprng = rand_core::OsRng;
        // let keypair = ed25519_dalek::Keypair::generate(&mut csprng);
        let keypair = test_keypair();
        let public_key = keypair.public.as_bytes();
        let secret_key = keypair.secret.as_bytes();
        println!("Public key: {:?}", bs58::encode(public_key).into_string());
        println!("Secret key: {:?}", bs58::encode(secret_key).into_string());
        let id = b58_sha256(public_key);
        set("db/system/auth/master".to_owned(), id,  public_key);
    }
}

#[test]
pub fn test_generate_first_master() {
    generate_first_master();
}
