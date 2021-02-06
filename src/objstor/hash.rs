use anyhow::Result;

pub fn hash_with_salt(password: &str, salt: &str) -> Result<String> {
    let hash = bcrypt::hash_with_salt(password, 12, salt.as_bytes())?;
    Ok(hash.to_string())
}
