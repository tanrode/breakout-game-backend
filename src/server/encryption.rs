use bcrypt::{hash, DEFAULT_COST, verify};

pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    // Hash password with default cost (12)
    let hashed_password = hash(password, DEFAULT_COST).unwrap();
    Ok(hashed_password)
}

pub fn verify_password(password: &str, hashed_password: &str) -> bool {
    match verify(password, hashed_password) {
        Ok(is_valid) => is_valid,
        Err(_) => false,
    }
}
