mod objstor;

use anyhow::Result;
use objstor::{backend::SqliteUserBackend, AppState, Config};
use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};
use tide::prelude::*;

#[async_std::main]
async fn main() -> Result<()> {
    let config = Config::new_from_env()?;
    let addr = config.get_addr()?;

    let pool = SqlitePool::connect(config.get_conn_str()).await?;
    let appstate = AppState {
        user: &SqliteUserBackend::new(&pool),
    };

    appstate.user.init().await?;

    let mut app = tide::new();

    app.at("/").get(|_| async { Ok("Welcome to objstor!") });

    let mut listener = app.bind(addr).await?;
    for info in listener.info().iter() {
        println!("Server listening on {}", info);
    }

    listener.accept().await?;
    Ok(())
}
