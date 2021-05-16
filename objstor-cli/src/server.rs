use crate::{config::Serve, state::State};
use anyhow::Result;
use sqlite_provider::SqliteObjstorProvider;
use tide::{prelude::*, Server};

pub async fn serve(s: &Serve) -> Result<()> {
    let app = get_app(&s).await?;

    println!("Initializing server...");
    let state = app.state();
    state.provider.init().await?;
    println!("Initializing server complete...");

    let mut listener = app.bind((&s.host, s.port)).await?;
    for info in listener.info().iter() {
        println!("Server listening on {}", info);
    }

    listener.accept().await?;

    Ok(())
}

async fn get_app(s: &Serve) -> Result<Server<State>> {
    tide::log::start();
    let state = get_state(&s).await?;
    let mut app = tide::with_state(state);
    app.at("/").get(|_| async { Ok("Hello from objstor!") });
    Ok(app)
}

async fn get_state(s: &Serve) -> Result<State> {
    let provider = SqliteObjstorProvider::connect(&s.connection_string).await?;
    Ok(State::new(Box::new(provider)))
}
