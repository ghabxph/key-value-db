use std::path::Path;
use std::fs;

use crate::{b58_sha256, DBError};

// Get value from database from given namespace
pub fn get(ns: String, key: String) -> Result<Vec<u8>, DBError> {

    // Convert key into sha256 base58 string
    let key = b58_sha256(key.as_bytes());

    let mut path = ns.to_owned();
    path.push('/');
    path.push_str(&key);
    path.push_str(".dat");
    let path = Path::new(&path);

    // Check if key exists
    if path.exists() == false {
        // Throw error
        Err(DBError::KeyDoesNotExist)
    } else {
        // Return path
        Ok(fs::read(path).unwrap())
    }
}
