use anyhow::Result;
use std::env;

pub struct Config {}

impl Config {
    pub fn new_from_env() -> Result<Self> {
        dotenv::dotenv().ok();
        Ok(Config {})
    }
}
