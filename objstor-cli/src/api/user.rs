use crate::state::State;
use objstor::User;
use serde::{Deserialize, Serialize};
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

#[derive(Debug, Deserialize, Serialize)]
pub struct ChangePasswordModel {
    pub username: String,
    pub current_password: String,
    pub new_password: String,
}

/// curl -X POST -i 'http://127.0.0.1:5000/api/password' --data '{"username":"admin","current_password": "admin", "new_password": "admin1"}'
pub async fn change_password(mut req: Request<State>) -> tide::Result {
    let model: ChangePasswordModel = req.body_json().await?;
    if model.new_password.len() < 5 {
        return Ok(Response::new(StatusCode::BadRequest));
    }

    let provider = &req.state().provider;
    if provider
        .validate_user(&model.username, &model.current_password)
        .await?
    {
        provider
            .update_user_password(&model.username, &model.new_password)
            .await?;
        Ok(Response::new(StatusCode::Ok))
    } else {
        Ok(Response::new(StatusCode::Unauthorized))
    }
}
