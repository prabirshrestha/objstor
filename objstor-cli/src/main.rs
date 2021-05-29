mod api;
mod config;
mod server;
mod state;

use anyhow::Result;
use config::Command;

#[async_std::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let config = config::parse();
    match &config.command {
        Command::Serve(s) => server::serve(&s).await?,
    }
    Ok(())
}
