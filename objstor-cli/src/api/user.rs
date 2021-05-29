use crate::state::State;
use tide::{Request, Response, StatusCode};

pub async fn create_user(_req: Request<State>) -> tide::Result {
    let res = Response::builder(StatusCode::NotImplemented).build();
    Ok(res)
}
