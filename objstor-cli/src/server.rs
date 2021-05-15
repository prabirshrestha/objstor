use crate::{config::Serve, state::State};
use anyhow::Result;
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
    tide::log::start();
    let state = get_state(&s).await?;
    let mut app = tide::with_state(state);
    app.at("/").get(|_| async { Ok("Hello from objstor!") });
    Ok(app)
}

async fn get_state(_s: &Serve) -> Result<State> {
    Ok(State {})
}
