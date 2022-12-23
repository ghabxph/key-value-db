use std::path::Path;
use std::fs;

use crate::b58_sha256;

// Set value from database from given namespace
pub fn set(ns: String, key: String, value: &[u8]) {

    // Convert key into sha256 base58 string
    let key = b58_sha256(key.as_bytes());

    let mut path = ns.to_owned();
    path.push('/');
    path.push_str(&key);
    path.push_str(".dat");
    let path = Path::new(&path);

    // Write the value to a file
    fs::write(path, value).unwrap();
}
