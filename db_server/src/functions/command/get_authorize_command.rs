use crate::{Command, split, Permission, PublicKey};

// Create get authorize command
pub fn get_authorize_command(message: &[u8]) -> Command {

    // Get tokens
    let mut tokens = split(message, b'\x0A');

    // Drop AUTHORIZE BY
    tokens.pop().unwrap();

    // Get permission
    let permission = tokens.pop().unwrap();
    let permission = get_permission_value(permission.as_slice()).unwrap();

    // Get public key token
    let public_key = tokens.pop().unwrap();
    let public_key = get_public_key_value(public_key.as_slice()).unwrap();

    Command::Authorize(public_key, permission)
}

fn get_permission_value(token: &[u8]) -> Result<Permission, GetPermissionValueError> {
    // Get tokens
    let tokens = split(token, b' ');

    // We expect AS token
    let token = String::from_utf8_lossy(tokens.get(0).unwrap()).to_string();
    if token.eq("AS") == false {
        return Err(GetPermissionValueError::ExpectsTokenAS)
    }

    // We expect MASTER | APPLICATION | READER, otherwise throw error.
    let token = String::from_utf8_lossy(tokens.get(1).unwrap()).to_string();
    if token.eq("MASTER") {
        return Ok(Permission::Master)
    } else if token.eq("APPLICATION") {
        return Ok(Permission::Application)
    } else if token.eq("READER") {
        return Ok(Permission::Reader)
    }
    else {
        return Err(GetPermissionValueError::ExpectsTokenPERMISSION)
    }
}

fn get_public_key_value(token: &[u8]) -> Result<PublicKey, GetPublicKeyValueError> {
    // Get tokens
    let tokens = split(token, b' ');

    // Expects PUBLIC KEY token
    let mut token = "".to_owned();
    token.push_str(&String::from_utf8_lossy(tokens.get(0).unwrap()).to_string());
    token.push(' ');
    token.push_str(&String::from_utf8_lossy(tokens.get(1).unwrap()).to_string());
    if token.eq("PUBLIC KEY") == false {
        return Err(GetPublicKeyValueError::ExpectsTokenPUBLICKEY)
    }

    // Decode public key bytes
    let token = tokens.get(2).unwrap();
    let token = split(token.as_slice(), b'"');
    let token = token.get(1).unwrap();
    let public_key = bs58::decode(token).into_vec().unwrap();

    return Ok(public_key.as_slice().try_into().unwrap());
}

#[derive(Debug)]
pub enum GetPermissionValueError {
    ExpectsTokenAS,
    ExpectsTokenPERMISSION
}

#[derive(Debug)]
pub enum GetPublicKeyValueError {
    ExpectsTokenPUBLICKEY
}