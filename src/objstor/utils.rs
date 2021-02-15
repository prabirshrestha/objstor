use anyhow::Result;
use uuid::Uuid;

pub fn uuid() -> Result<String> {
    Ok(Uuid::new_v4().to_string())
}

pub fn hash_with_salt(password: &str, salt: &str) -> Result<String> {
    let hash = bcrypt::hash_with_salt(password, 12, salt.as_bytes())?;
    Ok(hash.to_string())
}
