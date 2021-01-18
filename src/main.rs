mod objstor;

use anyhow::Result;
use objstor::{Config, User};
use tide::prelude::*;

#[async_std::main]
async fn main() -> Result<()> {
    let config = Config::new_from_env()?;
    let addr = config.get_addr()?;

    let app = tide::new();

    let mut listener = app.bind(addr).await?;
    for info in listener.info().iter() {
        println!("Server listening on {}", info);
    }

    listener.accept().await?;
    Ok(())
}
