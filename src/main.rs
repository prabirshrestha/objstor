mod objstor;

use anyhow::Result;
use objstor::User;

#[async_std::main]
async fn main() -> Result<()> {
    let app = tide::new();
    app.listen("127.0.0.1:3000").await?;
    Ok(())
}
