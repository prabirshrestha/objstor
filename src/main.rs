mod objstor;

use anyhow::Result;

#[async_std::main]
async fn main() -> Result<()> {
    objstor::run(&objstor::opt::parse()).await?;
    Ok(())
}
