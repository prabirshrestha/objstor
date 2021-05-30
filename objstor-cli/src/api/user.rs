use crate::state::State;
use objstor::User;
use tide::{Request, Response, StatusCode};

pub async fn create_user(req: Request<State>) -> tide::Result {
    if let Some(user) = req.ext::<User>() {
        if user.is_admin {
            let res = Response::builder(StatusCode::Ok).build();
            return Ok(res);
        }
    }

    let res = Response::builder(StatusCode::Unauthorized)
        .header("WWW-Authenticate", "Basic")
        .build();
    Ok(res)
}
