mod objstor;

use anyhow::Result;
use objstor::{Config, User};

#[async_std::main]
async fn main() -> Result<()> {
    let config = Config::new_from_env()?;
    let app = tide::new();
    app.listen("127.0.0.1:3000").await?;
    Ok(())
}
