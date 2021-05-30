use crate::state::State;
use objstor::User;
use tide::{Request, Response, StatusCode};

pub async fn create_user(req: Request<State>) -> tide::Result {
    if let Some(_user) = req.ext::<User>() {
        let res = Response::builder(StatusCode::Ok).build();
        Ok(res)
    } else {
        let res = Response::builder(StatusCode::Unauthorized)
            .header("WWW-Authenticate", "Basic")
            .build();
        Ok(res)
    }
}
