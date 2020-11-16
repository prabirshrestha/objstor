mod objstor;

use anyhow::Result;
use objstor::User;

#[async_std::main]
async fn main() -> Result<()> {
    println!("Hello, world!");
    Ok(())
}
