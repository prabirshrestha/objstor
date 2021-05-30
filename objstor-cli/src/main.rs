use anyhow::Result;
use objstor_cli::{
    config::{self, Command},
    server,
};

#[async_std::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let config = config::parse();
    match &config.command {
        Command::Serve(s) => server::serve(&s).await?,
    }
    Ok(())
}
