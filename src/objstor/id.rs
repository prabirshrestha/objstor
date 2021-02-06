use anyhow::Result;
use uuid::Uuid;

pub fn uuid() -> Result<String> {
    Ok(Uuid::new_v4().to_string())
}
