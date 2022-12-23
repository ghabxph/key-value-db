
pub enum Command {

    /// Grant permission to given public key
    Authorize([u8;32], Permission),

    /// Revoke access
    Revoke([u8;32]),

    /// Get value from given key
    Get(String),

    /// Set value from given key
    Set(String, Vec<u8>),
}

pub enum Permission {
    /// Master has full db permission and the right to revoke key
    Master,
    /// Application has read/write access to user space db
    Application,
    /// Reader has read access to user space db
    Reader,
}