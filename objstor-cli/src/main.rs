use anyhow::Result;

#[async_std::main]
async fn main() -> Result<()> {
    let mut app = tide::new();
    app.at("/").get(|_| async { Ok("Hello from objstor!") });
    app.listen("0.0.0.0:5000").await?;
    Ok(())
}
