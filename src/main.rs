mod objstor;

use std::sync::Arc;

use anyhow::Result;
use objstor::{backend::SqliteUserBackend, AppState, Config};
use sqlx::SqlitePool;
use tide::prelude::*;

#[async_std::main]
async fn main() -> Result<()> {
    objstor::run(&objstor::opt::parse()).await?;
    // let config = Config::new_from_env()?;
    // let addr = config.get_addr()?;

    // let pool = SqlitePool::connect(config.get_conn_str()).await?;

    // let appstate = AppState {
    //     userbackend: Arc::new(Box::new(SqliteUserBackend::new(
    //         Box::new(config),
    //         Box::new(pool),
    //     ))),
    // };
    // appstate.userbackend.init().await?;

    // let mut app = tide::with_state(appstate);
    // register_routes(&mut app);

    // let mut listener = app.bind(addr).await?;
    // for info in listener.info().iter() {
    //     println!("Server listening on {}", info);
    // }

    // listener.accept().await?;
    Ok(())
}

// fn register_routes(app: &mut tide::Server<AppState>) {
//     app.at("/").get(|_| async { Ok("Welcome to objstor!") });
//     app.at("/api/users/:id").get(|_| async { Ok("get user") });
//     app.at("/api/users").post(|_| async { Ok("create user") });
//     app.at("/api/users").put(|_| async { Ok("update user") });
//     app.at("/api/storage/:id")
//         .get(|_| async { Ok("get storage") });
//     app.at("/api/storage")
//         .post(|_| async { Ok("create storage") });
//     app.at("/api/storage")
//         .put(|_| async { Ok("update storage") });
//     app.at("/api/storage/:id")
//         .delete(|_| async { Ok("delete storage") });
// }
