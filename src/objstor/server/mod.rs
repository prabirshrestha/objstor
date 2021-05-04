pub mod state;
pub mod webdav;

use super::{
    backend::{sqlite::SqliteObjstorBackend, ObjstorBackend},
    opt::Serve,
};
use anyhow::Result;
use async_vfs::backend::OsFs;
use state::State;
use std::sync::Arc;
use tide::{prelude::*, Server, StatusCode};
use webdav::WebdavMiddleware;

pub async fn serve(s: &Serve) -> Result<()> {
    tide::log::start();

    let app = get_app(&s).await?;

    let mut listener = app.bind((&s.host, s.port)).await?;
    for info in listener.info().iter() {
        println!("Server listening on {}", info);
    }

    listener.accept().await?;

    Ok(())
}

async fn get_app(s: &Serve) -> Result<Server<State>> {
    let objstor_backend = SqliteObjstorBackend::new(&s.connection_string, s.salt.clone()).await?;
    objstor_backend.init().await?;
    let state = State::new(Arc::new(objstor_backend));
    let mut app = tide::with_state(state);
    app.at("/webdav")
        .with(WebdavMiddleware::new(Box::new(OsFs::new("./"))))
        .all(|_| async { Ok(StatusCode::BadRequest) });
    Ok(app)
}
