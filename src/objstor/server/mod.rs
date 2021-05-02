pub mod state;

use self::state::State;
use super::{
    backend::{sqlite::SqliteObjstorBackend, ObjstorBackend},
    opt::Serve,
};
use anyhow::Result;
use std::sync::Arc;
use tide::{prelude::*, Server};

pub async fn serve(s: &Serve) -> Result<()> {
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
    let app = tide::with_state(state);
    Ok(app)
}
