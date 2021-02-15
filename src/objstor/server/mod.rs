pub mod state;

use self::state::State;
use super::opt::Serve;
use anyhow::Result;
use tide::{prelude::*, Server};

pub async fn serve(s: &Serve) -> Result<()> {
    let app = get_app(&s).await?;

    let mut listener = app.bind(format!("0.0.0.0:{}", s.port)).await?;
    for info in listener.info().iter() {
        println!("Server listening on {}", info);
    }

    listener.accept().await?;

    Ok(())
}

async fn get_app(_s: &Serve) -> Result<Server<State>> {
    let state = State::default();
    let mut app = tide::with_state(state);
    Ok(app)
}

